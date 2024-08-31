//! [`crd`] controller implementation.

pub mod pcp;
pub mod reconciler;
pub mod status;

use std::sync::Arc;

use futures::StreamExt as _;

/// Run the controller until it exits.
pub async fn run(
    controller: kube::runtime::Controller<crd::PCPMap>,
    ctx: Arc<reconciler::Context>,
) {
    controller
        .run(reconciler::reconcile, reconciler::error_policy, ctx)
        .for_each(|result| async move {
            match result {
                Ok((obj, action)) => tracing::info!(message = "reconciled", ?obj, ?action),
                Err(error) => tracing::error!(message = "reconcile failed", ?error),
            }
        })
        .await;
}
