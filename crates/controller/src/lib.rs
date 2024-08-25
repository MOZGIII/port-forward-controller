//! [`PcpMap`] controller implementation.

use std::sync::Arc;

use crd::PcpMap;
use kube::runtime::{controller::Action, finalizer};
use port_control_client_core::map;

pub async fn reconciler<MapClient: map::Client>(
    obj: Arc<PcpMap>,
    ctx: Arc<MapClient>,
) -> Result<Action, Error> {
    finalizer(api, finalizer_name, obj, reconcile)
}

pub fn error_policy<S>(_obj: Arc<PcpMap>, _error: &Error, _ctx: Arc<S>) -> Action {
    Action::requeue(std::time::Duration::from_secs(60))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub async fn qwe(clinet: kube::Client) -> Result<(), ()> {
    kube::runtime::Controller::new(
        Api::<ConfigMap>::all(client.clone()),
        watcher::Config::default().labels("configmap-secret-syncer.nullable.se/sync=true"),
    )
    .run(
        |cm, _| {
            let ns = cm
                .meta()
                .namespace
                .as_deref()
                .ok_or(Error::NoNamespace)
                .unwrap();
            let cms: Api<ConfigMap> = Api::namespaced(client.clone(), ns);
            let secrets: Api<Secret> = Api::namespaced(client.clone(), ns);
            async move {
                finalizer(
                    &cms,
                    "configmap-secret-syncer.nullable.se/cleanup",
                    cm,
                    |event| async {
                        match event {
                            Event::Apply(cm) => apply(cm, &secrets).await,
                            Event::Cleanup(cm) => cleanup(cm, &secrets).await,
                        }
                    },
                )
                .await
            }
        },
        |_obj, _err, _| Action::requeue(Duration::from_secs(2)),
        Arc::new(()),
    )
    .for_each(|msg| async move { info!("Reconciled: {:?}", msg) })
    .await;
}
