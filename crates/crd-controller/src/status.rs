//! The status listener.

use std::collections::HashMap;

use futures::{Stream, StreamExt as _};

/// Indexer specialized for status listener.
pub mod indexer {
    /// An indexer that is specialized for status listener.
    pub type Indexer =
        indexer::Indexer<pcp_client::mapping::Id, crate::pcp::Converter, ValueExtractor>;

    /// An indexer reader that is specialized for status listener.
    pub type Reader<'a> =
        indexer::Reader<'a, pcp_client::mapping::Id, crate::pcp::Converter, ValueExtractor>;

    /// The [`indexer::ObjectRef`] extractor type.
    pub type ValueExtractor = fn(&crd::PCPMap) -> Option<indexer::ObjectRef>;

    /// The [`indexer::ObjectRef`] extractor fn.
    pub fn value_extactor(obj: &crd::PCPMap) -> Option<indexer::ObjectRef> {
        Some(indexer::ObjectRef {
            namespace: obj.metadata.namespace.clone()?,
            name: obj.metadata.name.clone()?,
        })
    }

    impl indexer::extractor::Key for crate::pcp::Converter {
        type Object = crd::PCPMap;
        type Key = pcp_client::mapping::Id;

        fn extract_key(&self, obj: &Self::Object) -> Option<Self::Key> {
            self.mapping_id_from_crd(obj).ok()
        }
    }

    /// Create a new indexer for a status listener.
    pub fn new(converter: crate::pcp::Converter) -> Indexer {
        Indexer::new(converter, value_extactor)
    }
}

pub use self::indexer::Indexer;

/// The status listener.
///
/// Relays mapping state notifications from the PCP client to the CRD status.
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Listener {
    /// Indexer for the CRDs.
    pub indexer: Indexer,

    /// The Kubernetes client for executing the operations.
    #[derivative(Debug = "ignore")]
    pub kube_client: kube::Client,
}

impl Listener {
    /// Handle a mapping status notification.
    async fn handle_notification(
        &self,
        index_reader: &indexer::Reader<'_>,
        incoming: pcp_client::mapping::Incoming,
    ) -> Result<(), kube::error::Error> {
        let id = incoming.id();

        let Some(kube_ref) = index_reader.get(&id) else {
            return Ok(());
        };

        let external_endpoint = std::net::SocketAddr::new(
            pcp_ip_conv::split(incoming.packet_opcode.assigned_external_ip_address),
            incoming.packet_opcode.assigned_external_port,
        );

        let api =
            kube::Api::<crd::PCPMap>::namespaced(self.kube_client.clone(), &kube_ref.namespace);

        let pp = kube::api::PatchParams {
            dry_run: false,
            force: false,
            field_manager: Some("port-forward-controller".into()),
            field_validation: Some(kube::api::ValidationDirective::Strict),
        };

        let patch = kube::api::Patch::Apply(crd::PCPMapStatus {
            external_endpoint: Some(external_endpoint),
            ..Default::default()
        });

        api.patch_status(&kube_ref.name, &pp, &patch).await?;

        Ok(())
    }

    /// Run the status listener lifecycle loop.
    pub async fn lifecycle_loop<Watcher>(
        mut self,
        watcher: Watcher,
        mut notifications_rx: tokio::sync::mpsc::Receiver<pcp_client::mapping::Incoming>,
    ) where
        Watcher: Stream<
                Item = Result<
                    kube::runtime::watcher::Event<crd::PCPMap>,
                    kube::runtime::watcher::Error,
                >,
            > + Send,
    {
        let mut stashed_notifications = HashMap::new();
        let mut watcher = std::pin::pin!(watcher);
        loop {
            tokio::select! {
                maybe_result = watcher.next() => {
                    let Some(result) = maybe_result else {
                        return;
                    };

                    let event = match result {
                        Ok(val) => val,
                        Err(error) => {
                            tracing::error!(message = "status listener watcher error", ?error);
                            continue;
                        }
                    };

                    self.indexer.handle_event(event);

                    if let Ok(reader) = self.indexer.reader() {
                        for (_, notification) in stashed_notifications.drain() {
                            if let Err(error) = self.handle_notification(&reader, notification).await {
                                tracing::error!(message = "error while handling stashed notification", ?error);
                            }
                        }
                    }
                }
                maybe_notification = notifications_rx.recv() => {
                    let Some(notification) = maybe_notification else {
                        return;
                    };

                    let Ok(reader) = self.indexer.reader() else {
                        stashed_notifications.insert(notification.id(), notification);
                        continue;
                    };

                    if let Err(error) = self.handle_notification(&reader, notification).await {
                        tracing::error!(message = "error while handling notification", ?error);
                    }
                }
            }
        }
    }
}
