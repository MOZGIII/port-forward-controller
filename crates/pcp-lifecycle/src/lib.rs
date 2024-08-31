#![allow(missing_docs, clippy::missing_docs_in_private_items)]

pub trait Mapping {
    /// Return true if this is the same mapping instance.
    ///
    /// The following fields must match:
    /// - Internal IP
    /// - Protocol
    /// - Internal Port
    /// - Nonce
    fn is_same_mapping_instance(&self, other: &Self) -> bool;
}

pub trait Incoming<Other> {
    /// Return true if this is the mapping for the same exposed resource.
    ///
    /// The following fields must match:
    /// - Internal IP
    /// - Protocol
    /// - Internal Port
    fn is_same_exposed_resource(&self, other: &Other) -> bool;
}

pub mod cleanup {
    pub trait Maybe {
        fn is_cleanup(&self) -> bool;
    }

    pub trait Into {
        type Mapping;

        fn into_cleanup(self) -> Self::Mapping;
    }
}

#[derive(Debug)]
pub struct State<RenewMapping, CleanupMapping, IncomingMapping>
where
    RenewMapping: Mapping + cleanup::Into<Mapping = CleanupMapping>,
    CleanupMapping: Mapping,
    IncomingMapping: cleanup::Maybe + Incoming<RenewMapping> + Incoming<CleanupMapping>,
{
    /// The desired state.
    ///
    /// Used to keep track of our prior demands to ensure we don't issue conflicting requests.
    desired: Option<RenewMapping>,

    /// The effective state that is send to and reported by the server.
    effective: Option<IncomingMapping>,

    /// The cleanup queue.
    cleanup_queue: Vec<CleanupMapping>,
}

#[derive(Debug)]
pub struct PendingActions<'a, RenewMapping, CleanupMapping> {
    pub renew: Option<&'a RenewMapping>,
    pub cleanup: &'a [CleanupMapping],
}

impl<RenewMapping, CleanupMapping, IncomingMapping>
    State<RenewMapping, CleanupMapping, IncomingMapping>
where
    RenewMapping: Mapping + cleanup::Into<Mapping = CleanupMapping>,
    CleanupMapping: Mapping,
    IncomingMapping: cleanup::Maybe + Incoming<RenewMapping> + Incoming<CleanupMapping>,
{
    pub fn new(mapping: RenewMapping) -> Self {
        let desired = Some(mapping);
        let effective = None;
        let cleanup_queue = Vec::default();
        Self {
            desired,
            effective,
            cleanup_queue,
        }
    }

    pub fn update_desired(&mut self, new_mapping: RenewMapping) -> UpdateDesiredOutcome {
        match self.desired {
            None => {
                self.desired = Some(new_mapping);
                UpdateDesiredOutcome::InPlace
            }
            Some(ref mut desired_mapping) => {
                let old_mapping = core::mem::replace(desired_mapping, new_mapping);
                if old_mapping.is_same_mapping_instance(desired_mapping) {
                    UpdateDesiredOutcome::InPlace
                } else {
                    let cleanup_mapping = old_mapping.into_cleanup();
                    self.cleanup_queue.push(cleanup_mapping);
                    UpdateDesiredOutcome::Recreated
                }
            }
        }
    }

    pub fn remove_desired(&mut self) -> RemoveDesiredOutcome {
        match self.desired.take() {
            None => RemoveDesiredOutcome::WasAbsent,
            Some(old_mapping) => {
                let cleanup_mapping = old_mapping.into_cleanup();
                self.cleanup_queue.push(cleanup_mapping);
                RemoveDesiredOutcome::Removed
            }
        }
    }

    pub fn pending_actions(&self) -> PendingActions<'_, RenewMapping, CleanupMapping> {
        PendingActions {
            renew: self.desired.as_ref(),
            cleanup: self.cleanup_queue.as_slice(),
        }
    }

    pub fn handle_server_notification(&mut self, incoming: IncomingMapping) {
        if incoming.is_cleanup() {
            self.cleanup_queue
                .retain(|cleanup| !incoming.is_same_exposed_resource(cleanup));
        }

        if let Some(desired) = &self.desired {
            if incoming.is_same_exposed_resource(desired) {
                self.effective = Some(incoming);
            }
        }

        if self.cleanup_queue.is_empty() && self.desired.is_none() {
            self.effective = None;
        }
    }

    pub fn desired(&self) -> Option<&RenewMapping> {
        self.desired.as_ref()
    }

    pub fn effective(&self) -> Option<&IncomingMapping> {
        self.effective.as_ref()
    }
}

/// The outcome of the [`State::update_desired`] call.
pub enum UpdateDesiredOutcome {
    /// The `desired` value was updated in-place.
    ///
    /// The state might not need to be reconciled.
    InPlace,

    /// The `desired` value was updated via re-creation.
    ///
    /// The state should be reconciled.
    Recreated,
}

/// The outcome of the [`State::remove_desired`] call.
pub enum RemoveDesiredOutcome {
    /// The `desired` value was removed from the state.
    ///
    /// The state should be reconciled.
    Removed,

    /// The `desired` value was already absent at the state, so no state changes
    /// actually occurred.
    ///
    /// The state might not need to be reconciled.
    WasAbsent,
}
