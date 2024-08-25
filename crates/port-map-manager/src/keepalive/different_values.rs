//! The keepalive loop for a single port map.

use std::borrow::Borrow;

use futures_core::Stream;
use port_control_client_core::map;
use tokio_stream::StreamExt as _;

use super::{Bucket, Params, Value};

enum State {}

pub async fn map<MapClient, OnUpdate>(
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
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);

        let handle = tokio::spawn(super::same_value::map(
            client, on_update, bucket, value, params, &mut rx,
        ));
    }
}
