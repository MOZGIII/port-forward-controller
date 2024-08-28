#![allow(missing_docs, clippy::missing_docs_in_private_items)]

pub mod option;

pub use pcp_primitives::*;

#[derive(Debug, Clone)]
pub struct Mapping {
    /// Lifetime (in seconds).
    ///
    /// Specified by the client.
    pub lifetime: LifetimeSeconds,

    /// The mapping nonce.
    ///
    /// A unique random sequence of bytes.
    pub nonce: Nonce,

    /// Protocol.
    ///
    /// Specified by the client, server must acceept or reject with `UNSUPP_PROTOCOL`.
    pub protocol: Protocol,

    /// Internal IP.
    ///
    /// Specified by the client, but can't be chosen freely as the server will check the packets
    /// it is receiving are sent from this `src`, and will also send the reply to that packet.
    pub internal_ip: Address,

    /// Internal port.
    ///
    /// Specified by the client, server must accept or reject with `UNSUPP_PROTOCOL`.
    pub internal_port: Port,

    /// External port.
    ///
    /// Specified by the server, but the client can suggest a port it wants.
    pub external_port: Port,

    /// External IP.
    ///
    /// Technically specified by the server, but the client can suggest the IP address it wants.
    pub exteranl_ip: Address,

    /// Third Party option.
    ///
    /// Specified by the client. Server must accept or ``
    ///
    /// Indicates this mapping does not have the Client IP as its Internal IP, but the server
    /// should respect that and create the port forward.
    ///
    /// This MUST NOT be implemented without a separate authorization.
    pub third_party: option::ThirdParty,

    /// Prefer Failure option.
    ///
    /// ...
    pub prefer_failure: option::PreferFailure,

    /// ...
    pub filters: option::Filters,
}

pub type MapRequest = Mapping;
pub type MapResponse = Mapping;

impl Mapping {
    pub fn is_same_exposed_resource(&self, other: &Self) -> bool {
        self.protocol == other.protocol
            && self.internal_ip == other.internal_ip
            && self.internal_port == other.internal_port
    }

    pub fn is_same_nonce(&self, other: &Self) -> bool {
        self.nonce == other.nonce
    }

    /// Used for correlating response with a previously sent request.
    ///
    /// `false` means a given response is not for the specified mapping, and vice versa.
    ///
    /// Commutative.
    pub fn is_same_mapping_instance(&self, other: &Self) -> bool {
        self.is_same_exposed_resource(other) && self.is_same_nonce(other)
    }

    pub fn is_cleanup(&self) -> bool {
        self.lifetime == 0
    }

    pub fn set_cleanup(&mut self) {
        self.lifetime = 0;
    }
}

#[derive(Debug)]
pub struct State {
    /// The desired state.
    ///
    /// Used to keep track of our prior demands to ensure we don't issue conflicting requests.
    desired: Option<Mapping>,

    /// The effective state that is send to and reported by the server.
    effective: Option<MapResponse>,

    /// The cleanup queue.
    cleanup_queue: Vec<Mapping>,
}

impl State {
    pub fn new(mapping: Mapping) -> Self {
        let desired = Some(mapping);
        let effective = None;
        let cleanup_queue = Vec::default();
        Self {
            desired,
            effective,
            cleanup_queue,
        }
    }

    pub fn update_desired(&mut self, new_mapping: Mapping) {
        match self.desired {
            None => self.desired = Some(new_mapping),
            Some(ref mut desired_mapping) => {
                let mut old_mapping = core::mem::replace(desired_mapping, new_mapping);
                if !old_mapping.is_same_mapping_instance(desired_mapping) {
                    old_mapping.set_cleanup();
                    self.cleanup_queue.push(old_mapping);
                }
            }
        }
    }

    pub fn pending_actions(&self) -> Vec<MapRequest> {
        let mut actions = Vec::new();

        if let Some(ref mapping) = self.desired {
            actions.push(mapping.clone());
        }

        for mapping in &self.cleanup_queue {
            actions.push(mapping.clone());
        }

        actions
    }

    pub fn handle_server_notification(&mut self, incoming: MapResponse) {
        if let Some(desired) = &self.desired {
            if desired.is_same_exposed_resource(&incoming) {
                self.effective = Some(incoming.clone());
            }
        }

        if !incoming.is_cleanup() {
            return;
        }

        self.cleanup_queue
            .retain(|cleanup| !cleanup.is_same_exposed_resource(&incoming));

        if self.cleanup_queue.is_empty() && self.desired.is_none() {
            self.effective = None;
        }
    }
}
