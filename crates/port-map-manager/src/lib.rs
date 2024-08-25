//! Managed mapped ports.

#![allow(missing_docs, clippy::missing_docs_in_private_items)]

mod bookkeeping;
pub mod keepalive;
pub mod managed;

use std::{collections::HashMap};

use port_control_client_core::{map, LifetimeSeconds, Port, Protocol};

#[derive(Debug)]
pub struct Registry {
    allocated_ports: HashMap<AllocationTarget, MappingSpec>,
}

#[derive(Debug)]
pub struct AllocationTarget {
    pub gateway_address: std::net::IpAddr,
    pub gateway_port: Port,
    pub protocol: Protocol,
}

#[derive(Debug)]
pub struct MappingSpec {
    pub node_port: Port,
    pub lifetime_seconds: LifetimeSeconds,
}

pub trait OnUpdate {
    type MapError;

    async fn on_update(&self, update: &Result<map::Response, Self::MapError>);
}

impl<MapClient: map::Client> Manager<MapClient> {
    pub fn allocate(&mut self, )

}

// impl<MapClient: map::Client> Manager<MapClient> {
//     pub fn map<OnUpdate>(&self, on_update: OnUpdate, req: map::Request)
//     where
//         OnUpdate: self::OnUpdate,
//     {
//         let mut managed = self.managed.lock().unwrap();
//         let vacant_entry = match managed.entry(req.gateway_port) {
//             hash_map::Entry::Occupied(entry) => {
//                 let old_state = entry.get();
//                 if old_state.matches(node_port) [

//                 ]
//             }
//             hash_map::Entry::Vacant(entry) => entry,
//         };

//         let lifetime = Arc::new(AtomicU32::new(req.lifetime));

//         let keepalive_params = KeepaliveParams {
//             error_long_sleep: todo!(),
//             error_short_sleep: todo!(),
//         };

//         let (keepalive_stop_s, keepalive_stop_r) = tokio_signal::channel();
//         let keepalive_handle = tokio::spawn(keepalive(
//             Arc::clone(&self.map_client),
//             req.protocol,
//             req.gateway_port,
//             req.node_port,
//             lifetime,
//             keepalive_params,
//             on_update,
//             keepalive_stop_r,
//         ));

//         let managed_state =
//             managed::State::new(node_port, keepalive_handle, keepalive_stop_s, lifetime);

//         vacant_entry.insert(value)
//     }
// }
