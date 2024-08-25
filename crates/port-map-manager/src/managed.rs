use std::sync::{
    atomic::{self, AtomicU32},
    Arc,
};

use port_control_client_core::LifetimeSeconds;

#[derive(Debug)]
pub struct State {
    keepalive_handle: tokio::task::JoinHandle<()>,
    keepalive_stop: tokio_signal::Sender,
    node_port: u16,
    lifetime: Arc<AtomicU32>,
}

impl Drop for State {
    fn drop(&mut self) {
        self.keepalive_handle.abort();
    }
}

impl State {
    pub fn new(
        node_port: u16,
        keepalive_handle: tokio::task::JoinHandle<()>,
        keepalive_stop: tokio_signal::Sender,
        lifetime: Arc<AtomicU32>,
    ) -> Self {
        Self {
            keepalive_handle,
            keepalive_stop,
            node_port,
            lifetime,
        }
    }

    pub fn matches(&self, node_port: u16) -> bool {
        self.node_port == node_port
    }

    pub fn update_lifetome(&self, new_lifetime: LifetimeSeconds) {
        self.lifetime.store(new_lifetime, atomic::Ordering::Release);
    }
}
