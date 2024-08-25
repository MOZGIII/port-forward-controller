//! The keepalive loop for a single port map.

use std::borrow::Borrow;

use futures_core::Stream;
use port_control_client_core::map;
use tokio_stream::StreamExt as _;

use super::{once, Bucket, Params, Value};

enum State {
    Continue(Params),
    CleanupAndExit,
}

pub async fn map<MapClient, OnUpdate>(
    client: impl Borrow<MapClient>,
    on_update: impl Borrow<OnUpdate>,
    bucket: impl Borrow<Bucket>,
    value: impl Borrow<Value>,
    initial_state: Params,
    state_changes: impl Stream<Item = Params>,
) where
    MapClient: map::Client,
    OnUpdate: crate::OnUpdate<MapError = MapClient::Error>,
{
    let client = client.borrow();
    let on_update = on_update.borrow();
    let bucket = bucket.borrow();
    let value = value.borrow();

    let mut state_changes = std::pin::pin!(state_changes);

    let mut next_delay = std::time::Duration::ZERO;
    let mut state = State::Continue(initial_state);

    loop {
        let state_change = tokio::select! {
            msg = state_changes.next() => Some(match msg {
                None => State::CleanupAndExit,
                Some(new_params) => State::Continue(new_params),
            }),
            _ = tokio::time::sleep(next_delay) => None,
        };

        if let Some(new_state) = state_change {
            state = new_state;
        }

        let params = match &state {
            State::Continue(params) => params,
            State::CleanupAndExit => {
                once::map::<MapClient, OnUpdate>(
                    client,
                    on_update,
                    bucket,
                    value,
                    Params { lifetime: 0 },
                )
                .await;
                break;
            }
        };

        next_delay =
            once::map::<MapClient, OnUpdate>(client, on_update, bucket, value, params).await
    }
}
