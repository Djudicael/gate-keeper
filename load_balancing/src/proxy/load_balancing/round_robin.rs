use std::{cell::RefCell, rc::Rc};

use crate::proxy::{backend::Backend, load_balancing_algorithm::LoadBalancingAlgorithm};

#[derive(Debug)]
pub struct RoundRobin {
    pub next_backend: u32,
}

impl LoadBalancingAlgorithm for RoundRobin {
    fn next_available_backend(
        &mut self,
        backends: &mut Vec<Rc<RefCell<Backend>>>,
    ) -> Option<Rc<RefCell<Backend>>> {
        let res = backends
            .get(self.next_backend as usize % backends.len())
            .map(|backend| (*backend).clone());

        self.next_backend = (self.next_backend + 1) % backends.len() as u32;
        res
    }
}
impl Default for RoundRobin {
    fn default() -> Self {
        Self::new()
    }
}

impl RoundRobin {
    pub fn new() -> Self {
        Self { next_backend: 0 }
    }
}
