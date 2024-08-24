//! PCP implementation details.

/// The result code of the PCP response.
pub type ResultCode = u8;

/// Result code consts.
pub mod result_code {
    use super::ResultCode;

    /// Success.
    pub const SUCCESS: ResultCode = 0;

    /// The version number at the start of the PCP Request
    ///  header is not recognized by this PCP server.  This is a long
    ///  lifetime error.  This document describes PCP version 2.
    pub const UNSUPP_VERSION: ResultCode = 1;

    /// The requested operation is disabled for this PCP
    /// client, or the PCP client requested an operation that cannot be
    /// fulfilled by the PCP server's security policy.  This is a long
    /// lifetime error.
    pub const NOT_AUTHORIZED: ResultCode = 2;

    /// The request could not be successfully parsed.
    /// This is a long lifetime error.
    pub const MALFORMED_REQUEST: ResultCode = 3;

    /// Unsupported Opcode.  This is a long lifetime error.
    pub const UNSUPP_OPCODE: ResultCode = 4;

    /// Unsupported option.  This error only occurs if the
    /// option is in the mandatory-to-process range.  This is a long
    /// lifetime error.
    pub const UNSUPP_OPTION: ResultCode = 5;

    /// Malformed option (e.g., appears too many times,
    /// invalid length).  This is a long lifetime error.
    pub const MALFORMED_OPTION: ResultCode = 6;

    /// The PCP server or the device it controls is
    /// experiencing a network failure of some sort (e.g., has not yet
    /// obtained an external IP address).  This is a short lifetime error.
    pub const NETWORK_FAILURE: ResultCode = 7;

    /// Request is well-formed and valid, but the server has
    /// insufficient resources to complete the requested operation at this
    /// time.  For example, the NAT device cannot create more mappings at
    /// this time, is short of CPU cycles or memory, or is unable to
    /// handle the request due to some other temporary condition.  The
    /// same request may succeed in the future.  This is a system-wide
    /// error, different from USER_EX_QUOTA.  This can be used as a catch-
    /// all error, should no other error message be suitable.  This is a
    /// short lifetime error.
    pub const NO_RESOURCES: ResultCode = 8;

    /// Unsupported transport protocol, e.g., SCTP in a
    /// NAT that handles only UDP and TCP.  This is a long lifetime error.
    pub const UNSUPP_PROTOCOL: ResultCode = 9;

    /// This attempt to create a new mapping would exceed
    /// this subscriber's port quota.  This is a short lifetime error.
    pub const USER_EX_QUOTA: ResultCode = 10;

    /// The suggested external port and/or
    /// external address cannot be provided.  This error MUST only be
    /// returned for:
    /// *  MAP requests that included the PREFER_FAILURE option
    ///    (normal MAP requests will return an available external port)
    /// *  MAP requests for the SCTP protocol (PREFER_FAILURE is implied)
    /// *  PEER requests for details of the PREFER_FAILURE Option.  The
    ///    error lifetime depends on the reason for the failure.
    pub const CANNOT_PROVIDE_EXTERNAL: ResultCode = 11;

    /// The source IP address of the request packet does
    /// not match the contents of the PCP Client's IP Address field, due
    /// to an unexpected NAT on the path between the PCP client and the
    /// PCP-controlled NAT or firewall.  This is a long lifetime error.
    pub const ADDRESS_MISMATCH: ResultCode = 12;

    /// The PCP server was not able to create the
    /// filters in this request.  This result code MUST only be returned
    /// if the MAP request contained the FILTER option.  See Section 13.3
    /// for details of the FILTER Option.  This is a long lifetime error.
    pub const EXCESSIVE_REMOTE_PEERS: ResultCode = 13;
}
