//! The keepalive loop for a single port map.

pub mod different_values;
pub mod once;
pub mod same_value;

use std::borrow::Borrow;

use futures_core::Stream;
use port_control_client_core::{map, LifetimeSeconds};
use tokio_stream::StreamExt as _;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bucket {
    pub protocol: port_control_client_core::Protocol,
    pub gateway_port: port_control_client_core::Port,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Value {
    node_port: port_control_client_core::Port,
}

#[derive(Debug)]
pub struct Params {
    lifetime: port_control_client_core::LifetimeSeconds,
}

pub fn lifetime_to_duration(lifetime: LifetimeSeconds) -> std::time::Duration {
    let capped = std::cmp::max(lifetime, 1);
    std::time::Duration::from_secs(capped.into())
}

pub async fn map_with_changing_values<MapClient, OnUpdate>(
    client: impl Borrow<MapClient>,
    on_update: impl Borrow<OnUpdate>,
    bucket: impl Borrow<Bucket>,
    initial_value: Value,
    initial_params: Params,
    state_changes: impl Stream<Item = (Value, Params)>,
) where
    MapClient: map::Client,
    OnUpdate: crate::OnUpdate<MapError = MapClient::Error>,
{
    let client = client.borrow();
    let on_update = on_update.borrow();
    let bucket = bucket.borrow();

    let value = initial_value;
    let params = initial_params;

    loop {
        tokio::select! {
            state_change = state_changes.next() => {

            }
        }
    }
}
