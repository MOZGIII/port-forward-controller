pub mod request {
    use pcp_primitives::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Header {
        pub requested_lifetime: LifetimeSeconds,
        pub client_ip_address: Address,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Map {
        pub mapping_nonce: Nonce,
        pub protocol: Protocol,
        pub internal_port: Port,
        pub suggested_external_port: Port,
        pub suggested_external_ip_address: Address,
    }
}

pub mod response {
    use pcp_primitives::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Header {
        pub result_code: ResultCode,
        pub lifetime: LifetimeSeconds,
        pub epoch_time: EpochTime,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Map {
        pub mapping_nonce: Nonce,
        pub protocol: Protocol,
        pub internal_port: Port,
        pub assigned_external_port: Port,
        pub assigned_external_ip_address: Address,
    }
}
