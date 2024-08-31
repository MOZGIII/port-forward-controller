//! Reconciler.

use std::sync::Arc;

use kube::runtime::{controller::Action, finalizer};

use crate::pcp;

/// The name of the finalizer.
const DEFAULT_FINALIZER_NAME: &str = "port-forward-controller.io/cleanup";

/// The execution params of a reconciler.
#[derive(Debug)]
pub struct Params {
    /// The finalizer name to use.
    pub finalizer_name: Arc<str>,

    /// The time in which the sending of PCP client client into the command channel must complete.
    ///
    /// This can only stall if the PCP client lifecycele loop is stuck for too long, or if the
    /// contention of the commands it too high.
    pub pcp_client_command_timeout: std::time::Duration,

    /// The duration after which to requeue the resource to check on the cleanup process.
    ///
    /// The cleanup stalls the finalizer removal, and therefore resource deletion.
    pub cleanup_requeue_duration: std::time::Duration,

    /// The duration after which to requeue the resource after an error during reconciliation.
    pub error_requeue_duration: std::time::Duration,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            finalizer_name: DEFAULT_FINALIZER_NAME.into(),
            pcp_client_command_timeout: std::time::Duration::from_secs(60),
            cleanup_requeue_duration: std::time::Duration::from_secs(10),
            error_requeue_duration: std::time::Duration::from_secs(60),
        }
    }
}

/// The context of the reconciler.
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Context {
    /// The execution params of the reconciler.
    pub params: Params,

    /// PCP client command channel sender.
    pub command_tx: tokio::sync::mpsc::Sender<pcp_client::Command>,

    /// The Kubernetes API client for controlled resources.
    #[derivative(Debug = "ignore")]
    pub k8s_client: kube::Client,

    /// The converter for the CRD and PCP types.
    pub converter: pcp::Converter,
}

/// Apply the mapping update that happened at the API to the PCP client state.
pub async fn apply(obj: Arc<crd::PCPMap>, ctx: Arc<Context>) -> Result<Action, Error> {
    let mapping = ctx
        .converter
        .mapping_from_crd(&obj)
        .map_err(Error::Converter)?;
    ctx.command_tx
        .send_timeout(
            pcp_client::Command::UpsertDesired(mapping),
            ctx.params.pcp_client_command_timeout,
        )
        .await?;

    Ok(Action::requeue(std::time::Duration::from_secs(60)))
}

/// Run the cleanup process from the mapping at the PCP client in response to the resource
/// deletion at the API.
pub async fn cleanup(obj: Arc<crd::PCPMap>, ctx: Arc<Context>) -> Result<Action, Error> {
    let id = ctx
        .converter
        .mapping_id_from_crd(&obj)
        .map_err(Error::Converter)?;
    ctx.command_tx
        .send_timeout(
            pcp_client::Command::RemoveDesired(id),
            ctx.params.pcp_client_command_timeout,
        )
        .await?;

    let (tx, rx) = tokio::sync::oneshot::channel();
    ctx.command_tx
        .send_timeout(
            pcp_client::Command::HasState(id, tx),
            ctx.params.pcp_client_command_timeout,
        )
        .await?;

    let has_state = rx.await.map_err(Error::ReplyRxClosed)?;

    if has_state {
        Err(Error::CleanUpInProgress)
    } else {
        Ok(Action::await_change())
    }
}

/// Reconcile the changes at the API.
pub async fn reconcile(
    obj: Arc<crd::PCPMap>,
    ctx: Arc<Context>,
) -> Result<Action, finalizer::Error<Error>> {
    let client = ctx.k8s_client.clone();
    let maybe_namespace = obj.metadata.namespace.as_deref();
    let api = match maybe_namespace {
        Some(namespace) => kube::Api::namespaced(client, namespace),
        None => kube::Api::all(client),
    };
    finalizer(&api, &ctx.params.finalizer_name, obj, {
        let ctx = Arc::clone(&ctx);
        |event| async move {
            match event {
                finalizer::Event::Apply(obj) => apply(obj, ctx).await,
                finalizer::Event::Cleanup(obj) => cleanup(obj, ctx).await,
            }
        }
    })
    .await
}

/// Apply the error.
pub fn error_policy(
    _obj: Arc<crd::PCPMap>,
    error: &finalizer::Error<Error>,
    ctx: Arc<Context>,
) -> Action {
    match error {
        finalizer::Error::CleanupFailed(Error::CleanUpInProgress) => {
            Action::requeue(ctx.params.cleanup_requeue_duration)
        }
        _ => Action::requeue(ctx.params.error_requeue_duration),
    }
}

/// An error that can occur while reconciling.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Sending a command to the PCP client failed.
    #[error("unable to send a PCP client command: {0}")]
    CommandSend(#[from] tokio::sync::mpsc::error::SendTimeoutError<pcp_client::Command>),

    /// Conversion of the CRD into PCP type has failed.
    #[error("unable to covert the CRD into PCP type: {0}")]
    Converter(pcp::ConversionError),

    /// Waiting for the PCP client state reporting failed.
    #[error("PCP client response not delivered: {0}")]
    ReplyRxClosed(tokio::sync::oneshot::error::RecvError),

    /// An error to stop the finalizer from being removed.
    #[error("the cleanup is still in progress")]
    CleanUpInProgress,
}
