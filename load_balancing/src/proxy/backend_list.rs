use std::{cell::RefCell, rc::Rc};

use super::{
    backend::Backend,
    load_balancing::{random::Random, round_robin::RoundRobin},
    load_balancing_algorithm::{LoadBalancingAlgorithm, LoadBalancingAlgorithms},
};

#[derive(Debug)]
pub struct BackendList {
    pub backends: Vec<Rc<RefCell<Backend>>>,
    pub next_id: u32,
    pub load_balancing: Box<dyn LoadBalancingAlgorithm>,
}

impl BackendList {
    pub fn new() -> BackendList {
        BackendList {
            backends: Vec::new(),
            next_id: 0,
            load_balancing: Box::new(Random),
        }
    }

    pub fn import_configuration_state(backend_vec: &[Backend]) -> BackendList {
        let mut list = BackendList::new();
        for backend in backend_vec {
            let backend = Backend::new(
                backend.address,
                backend.load_balancing_parameters.clone(),
                backend.backup,
            );
            list.add_backend(backend);
        }

        list
    }

    pub fn add_backend(&mut self, backend: Backend) {
        let backend = Rc::new(RefCell::new(backend));
        self.backends.push(backend);
        self.next_id += 1;
    }

    pub fn available_backends(&mut self, backup: bool) -> Vec<Rc<RefCell<Backend>>> {
        self.backends
            .iter()
            .filter(|backend| {
                let owned = backend.borrow();
                owned.backup == Some(backup)
            })
            .map(Clone::clone)
            .collect()
    }

    pub fn next_available_backend(&mut self) -> Option<Rc<RefCell<Backend>>> {
        let mut backends = self.available_backends(false);

        if backends.is_empty() {
            backends = self.available_backends(true);
        }

        if backends.is_empty() {
            return None;
        }

        self.load_balancing.next_available_backend(&mut backends)
    }
    pub fn set_load_balancing_policy(&mut self, load_balancing_policy: LoadBalancingAlgorithms) {
        match load_balancing_policy {
            LoadBalancingAlgorithms::RoundRobin => {
                self.load_balancing = Box::new(RoundRobin::new())
            }
            LoadBalancingAlgorithms::Random => self.load_balancing = Box::new(Random {}),
        }
    }
}
