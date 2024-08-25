//! The keepalive loop for a single port map.

use std::borrow::Borrow;

use port_control_client_core::{map, ErrorLifetime};

use super::{lifetime_to_duration, Bucket, Params, Value};

pub async fn map<MapClient, OnUpdate>(
    client: impl Borrow<MapClient>,
    on_update: impl Borrow<OnUpdate>,
    bucket: impl Borrow<Bucket>,
    value: impl Borrow<Value>,
    params: impl Borrow<Params>,
) -> std::time::Duration
where
    MapClient: map::Client,
    OnUpdate: crate::OnUpdate<MapError = MapClient::Error>,
{
    let client = client.borrow();
    let on_update = on_update.borrow();
    let bucket = bucket.borrow();
    let value = value.borrow();
    let params = params.borrow();

    let result = client
        .map(map::Request {
            protocol: bucket.protocol,
            node_port: value.node_port,
            gateway_port: bucket.gateway_port,
            lifetime: params.lifetime,
        })
        .await;
    on_update.on_update(&result).await;

    match result {
        Ok(res) => {
            let half_lifetime = res.lifetime >> 1;
            lifetime_to_duration(half_lifetime)
        }
        Err(error) => {
            if let Some(error_lifetime) = error.lifetime() {
                lifetime_to_duration(error_lifetime)
            } else {
                std::time::Duration::from_secs(1)
            }
        }
    }
}
