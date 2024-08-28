use crate::{Address, PrefixLength};

#[derive(Debug, Clone)]
pub struct PcpOption<T> {
    /// Is the handling of this option optional.
    ///
    /// See <https://datatracker.ietf.org/doc/html/rfc6887#section-7.3>.
    ///
    /// Sets the most signiificant bit of the option code.
    pub is_optional: bool,

    /// The payload of the option.
    pub payload: T,
}

pub type ThirdParty = PcpOption<Address>;
pub type PreferFailure = PcpOption<()>;

pub type Filter = (Address, PrefixLength);
pub type Filters = PcpOption<Vec<Filter>>;
