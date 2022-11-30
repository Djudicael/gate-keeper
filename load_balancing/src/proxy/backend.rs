use std::net::SocketAddr;

#[derive(Debug)]
pub enum BackendStatus {
    Normal,
    Closing,
    Closed,
}

#[derive(Debug, Clone, Copy)]
pub struct LoadBalancingParams {
    pub weight: u8,
}

#[derive(Debug)]
pub struct Backend {
    pub address: SocketAddr,
    pub status: BackendStatus,
    pub failures: usize,
    pub load_balancing_parameters: Option<LoadBalancingParams>,
    pub backup: Option<bool>,
}

impl Backend {
    pub fn new(
        address: SocketAddr,
        load_balancing_parameters: Option<LoadBalancingParams>,
        backup: Option<bool>,
    ) -> Backend {
        Backend {
            address,
            status: BackendStatus::Normal,
            failures: 0,
            load_balancing_parameters,
            backup: backup,
        }
    }
}
