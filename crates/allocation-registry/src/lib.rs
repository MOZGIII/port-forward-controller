//! Allocation registry.
//!
//! See [`AllocationRegistry`].

use std::collections::{hash_map, HashMap};

/// The port representation.
type Port = u16;

/// The protocol representation.
type Protocol = u8;

/// The unified IPv4 and IPv6 address representation.
///
/// IPv4 addresses are represented via IPv4-mapped IPv6 addresses.
/// For our purposes it does not make sense to distinguish between the two.
type Address = core::net::Ipv6Addr;

/// Allocation registry.
///
/// Keeps track of port allocations to enable detection of the conflicting rules.
///
/// Bookkeeping is performed in the context of a fixed NAT gateway - so this
/// registry exists in one instance per PCP server.
#[derive(Debug)]
pub struct AllocationRegistry(HashMap<Key, Value>);

/// A key for an entry in the registry,
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Key {
    /// The protocol.
    pub protocol: Protocol,

    /// The external port.
    pub external_port: Port,
}

/// A value for an entry in the registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value {
    /// The internal address.
    pub internal_address: Address,

    /// The internal port.
    pub internal_port: Port,
}

/// An entry in the registry.
#[derive(Debug)]
pub struct Entry {
    /// The key.
    pub key: Key,

    /// The value.
    pub value: Value,
}

impl Key {
    /// Create a new [`Key`].
    #[inline]
    pub fn new(protocol: impl Into<Protocol>, external_port: impl Into<Port>) -> Self {
        Self {
            external_port: external_port.into(),
            protocol: protocol.into(),
        }
    }
}

impl Value {
    /// Create a new [`Value`].
    #[inline]
    pub fn new(internal_address: impl Into<Address>, internal_port: impl Into<Port>) -> Self {
        Self {
            internal_address: internal_address.into(),
            internal_port: internal_port.into(),
        }
    }
}

impl Entry {
    /// Create a new [`Entry`].
    #[inline]
    pub fn new(
        protocol: impl Into<Protocol>,
        external_port: impl Into<Port>,
        internal_address: impl Into<Address>,
        internal_port: impl Into<Port>,
    ) -> Self {
        let key = Key::new(protocol, external_port);
        let value = Value::new(internal_address, internal_port);

        Self { key, value }
    }

    /// Create a new [`Entry`] from the key and value.
    #[inline]
    pub fn from_kv(key: impl Into<Key>, value: impl Into<Value>) -> Self {
        let key = key.into();
        let value = value.into();

        Self { key, value }
    }
}

impl Default for AllocationRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AllocationRegistry {
    /// Create an empty [`AllocationRegistry`].
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Register an entry.
    ///
    /// Conflicting enties are not registered and an error it returned.
    pub fn register(
        &mut self,
        entry: impl Into<Entry>,
    ) -> Result<RegistrationSuccess, RegistrationConflict> {
        let Entry { key, value } = entry.into();

        match self.0.entry(key) {
            hash_map::Entry::Occupied(entry) => {
                let existing_value = entry.get();
                if existing_value == &value {
                    Ok(RegistrationSuccess::AlreadyExists)
                } else {
                    Err(RegistrationConflict(existing_value.clone()))
                }
            }
            hash_map::Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(RegistrationSuccess::Registered)
            }
        }
    }

    /// Register an entry, evicting a conflicnting entry if any.
    ///
    /// In case of conflict, new entry will end up in the registry, and the conflicting entry will
    /// be taken out and returned.
    pub fn force_register(&mut self, entry: impl Into<Entry>) -> ForceRegistrationResult {
        let Entry { key, value } = entry.into();

        match self.0.entry(key) {
            hash_map::Entry::Occupied(entry) => {
                let existing_value = entry.into_mut();
                if existing_value == &value {
                    ForceRegistrationResult::AlreadyExists
                } else {
                    let result =
                        ForceRegistrationResult::EvictedConflicting(existing_value.clone());
                    *existing_value = value;
                    result
                }
            }
            hash_map::Entry::Vacant(entry) => {
                entry.insert(value);
                ForceRegistrationResult::Registered
            }
        }
    }

    /// Remove the given key from the registry.
    pub fn unregister(&mut self, key: impl Into<Key>) -> Option<Value> {
        let key = key.into();
        self.0.remove(&key)
    }

    /// Remove the given key from the registry.
    pub fn compare_and_unregister(
        &mut self,
        entry: impl Into<Entry>,
    ) -> Result<(), CompareAndUnregisterError> {
        let Entry { key, value } = entry.into();

        match self.0.entry(key) {
            hash_map::Entry::Occupied(entry) => {
                let existing_value = entry.get();
                if existing_value == &value {
                    entry.remove();
                    Ok(())
                } else {
                    Err(CompareAndUnregisterError::DifferentValue(
                        existing_value.clone(),
                    ))
                }
            }
            hash_map::Entry::Vacant(_) => Err(CompareAndUnregisterError::KeyNotFound),
        }
    }
}

/// The successful registration details.
#[derive(Debug)]
pub enum RegistrationSuccess {
    /// A new entry was created.
    Registered,

    /// An entry with a non-conflicting value already existed.
    AlreadyExists,
}

/// The registration conflict information.
#[derive(Debug, thiserror::Error)]
#[error("registration conflict: {0}")]
pub struct RegistrationConflict(Value);

/// The successful force registration details.
#[derive(Debug)]
pub enum ForceRegistrationResult {
    /// A new entry was created.
    Registered,

    /// An entry with a non-conflicting value already existed.
    AlreadyExists,

    /// An entry with a conflicting value already existed.
    EvictedConflicting(Value),
}

/// The registration conflict information.
#[derive(Debug, thiserror::Error)]
pub enum CompareAndUnregisterError {
    /// Key not found in the registry.
    #[error("key not found in the registry")]
    KeyNotFound,

    /// Key found, but has a different value.
    #[error("registry has a different value for the specified key: {0}")]
    DifferentValue(Value),
}

/// Format the IP address as either IPv4 or IPv6.
fn format_address(addr: &Address) -> impl core::fmt::Display {
    if let Some(addr) = addr.to_ipv4_mapped() {
        return core::net::IpAddr::V4(addr);
    }
    core::net::IpAddr::V6(*addr)
}

impl core::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}:{}",
            format_address(&self.internal_address),
            self.internal_port,
        ))
    }
}
