//! The PCP client.

#![allow(missing_docs, clippy::missing_docs_in_private_items)]

pub mod mapping;
pub use mapping::Mapping;

use std::{
    collections::{hash_map, HashMap, HashSet},
    future::Future,
};

#[derive(Debug)]
pub struct Client<Runtime, Transport> {
    pub runtime: Runtime,
    pub transport: Transport,
    pub server_address: std::net::SocketAddr,
    pub mappings: HashMap<mapping::Id, pcp_lifecycle::State<Mapping, Mapping, mapping::Incoming>>,
    pub keepalive_interval: std::time::Duration,
    pub notifications_tx: tokio::sync::mpsc::Sender<mapping::Incoming>,
}

#[derive(Debug)]
pub enum Command {
    UpsertDesired(Mapping),
    RemoveDesired(mapping::Id),
    HasState(mapping::Id, tokio::sync::oneshot::Sender<bool>),
    GetEffective(
        mapping::Id,
        tokio::sync::oneshot::Sender<Option<mapping::Incoming>>,
    ),
}

impl<Runtime, Transport> Client<Runtime, Transport>
where
    Runtime: pcp_client_core::Runtime,
    Transport: pcp_client_core::Transport,
{
    async fn reconcile_once(&mut self) {
        let mut cleanups = Vec::new();
        let mut renews = Vec::new();
        let mut ids_to_remove = HashSet::new();

        for (&id, state) in &self.mappings {
            let pcp_lifecycle::PendingActions { renew, cleanup } = state.pending_actions();
            if !cleanup.is_empty() {
                cleanups.push((id, cleanup));
            }
            if let Some(renew) = renew {
                renews.push((id, renew));
            }

            if renew.is_none() && cleanup.is_empty() {
                ids_to_remove.insert(id);
            }
        }

        let ops = cleanups
            .into_iter()
            .flat_map(|(id, items)| items.iter().map(move |val| (id, val)));
        let ops = ops.chain(renews.into_iter());

        let mut packet = [0; pcp_packet::LEN];

        for (id, op) in ops {
            let Mapping {
                id:
                    mapping::Id {
                        protocol,
                        internal_ip,
                        internal_port,
                        nonce,
                    },
                params:
                    mapping::Params {
                        lifetime,
                        external_port,
                        exteranl_ip,
                        third_party: _,
                        prefer_failure,
                        filters: _,
                    },
            } = op;

            let enc = pcp_codec::encode::State::new(&mut packet).request().map(
                pcp_codec::data::request::Header {
                    requested_lifetime: *lifetime,
                    client_ip_address: *internal_ip,
                },
                pcp_codec::data::request::Map {
                    mapping_nonce: *nonce,
                    protocol: *protocol,
                    internal_port: *internal_port,
                    suggested_external_port: *external_port,
                    suggested_external_ip_address: *exteranl_ip,
                },
            );

            let request = if let Some(prefer_failure) = prefer_failure {
                let mut option_code = pcp_consts::option::PREFER_FAILURE;
                if prefer_failure.is_optional {
                    option_code |= 0b1000_0000;
                }
                enc.add_option(option_code, &[]).finish()
            } else {
                enc.finish()
            };

            if let Err(error) = self.transport.send(self.server_address, request).await {
                tracing::error!(message = "error while sending PCP packet", ?error);
                ids_to_remove.remove(&id);
            }
        }

        for id in ids_to_remove {
            self.mappings.remove(&id);
        }
    }

    async fn apply_incoming(
        &mut self,
        packet: &pcp_packet::Buffer,
        received_on: pcp_primitives::Address,
    ) {
        let incoming = pcp_codec::decode::State::new(packet);

        let Some((header, opcode)) = incoming.map_response_data() else {
            tracing::warn!(
                message = "unexpected non-MAP-response packet received",
                ?packet,
                %received_on,
            );
            return;
        };

        let incoming = mapping::Incoming {
            received_on,
            packet_header: header,
            packet_opcode: opcode,
        };

        self.runtime
            .spawn_background(self.notify_about_incoming(incoming));

        let id = incoming.id();

        let Some(state) = self.mappings.get_mut(&id) else {
            tracing::warn!(
                message = "received a server notification a mapping that is not in the lifecycle",
                ?incoming,
                ?packet,
                ?id,
            );
            return;
        };

        state.handle_server_notification(incoming);

        self.reconcile_once().await
    }

    fn notify_about_incoming(
        &self,
        value: mapping::Incoming,
    ) -> impl Future<Output = ()> + Send + 'static {
        let tx = self.notifications_tx.clone();
        async move {
            let _ = tx.send(value).await;
        }
    }

    async fn upsert_desired(&mut self, mapping: Mapping) {
        match self.mappings.entry(mapping.id) {
            hash_map::Entry::Occupied(mut entry) => {
                let _ = entry.get_mut().update_desired(mapping);
            }
            hash_map::Entry::Vacant(entry) => {
                entry.insert(pcp_lifecycle::State::new(mapping));
            }
        }

        self.reconcile_once().await;
    }

    async fn remove_desired(&mut self, id: mapping::Id) {
        let Some(state) = self.mappings.get_mut(&id) else {
            return;
        };

        let _ = state.remove_desired();
        self.reconcile_once().await;
    }

    fn has_state(&self, id: mapping::Id) -> bool {
        self.mappings.contains_key(&id)
    }

    fn effective_mapping(&self, id: mapping::Id) -> Option<&mapping::Incoming> {
        let state = self.mappings.get(&id)?;
        state.effective()
    }

    async fn handle_command(&mut self, command: Command) {
        match command {
            Command::UpsertDesired(mapping) => self.upsert_desired(mapping).await,
            Command::RemoveDesired(id) => self.remove_desired(id).await,
            Command::HasState(id, tx) => {
                let _ = tx.send(self.has_state(id));
            }
            Command::GetEffective(id, tx) => {
                let _ = tx.send(self.effective_mapping(id).cloned());
            }
        }
    }

    pub async fn lifecycle_loop(&mut self, mut rx: tokio::sync::mpsc::Receiver<Command>) {
        let mut next_keepalive = Box::pin(self.runtime.sleep(self.keepalive_interval));
        let mut incoming_packet = [0; pcp_packet::LEN];

        tracing::info!(message = "lifecycle loop started");

        loop {
            let next_incoming = self.transport.recv(&mut incoming_packet);

            tokio::select! {
                _ = &mut next_keepalive => {
                    tracing::info!(message = "keepalive timer triggered");
                    next_keepalive = Box::pin(self.runtime.sleep(self.keepalive_interval));
                    self.reconcile_once().await;
                },
                result = next_incoming => {
                    let recv_info = match result {
                        Ok(val) => val,
                        Err(error) => {
                            tracing::error!(message = "error while receiving a PCP packet", ?error);
                            break;
                        }
                    };
                    tracing::info!(message = "received PCP packet", ?recv_info);

                    self.apply_incoming(&incoming_packet, pcp_ip_conv::unify(recv_info.dst.ip())).await;
                }
                command = rx.recv() => {
                    let Some(command) = command else {
                        tracing::info!(message = "command channel rx closed");
                        break;
                    };
                    tracing::info!(message = "received command", ?command);

                    self.handle_command(command).await;
                }
            }
        }

        tracing::info!(message = "lifecycle loop ended");
    }

    pub async fn into_lifecycle_loop(mut self, rx: tokio::sync::mpsc::Receiver<Command>) {
        self.lifecycle_loop(rx).await
    }
}
