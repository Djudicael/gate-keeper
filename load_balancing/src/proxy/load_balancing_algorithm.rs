use std::{cell::RefCell, fmt::Debug, rc::Rc};

use super::backend::Backend;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]

pub enum LoadBalancingAlgorithms {
    RoundRobin,
    Random,
}

impl Default for LoadBalancingAlgorithms {
    fn default() -> Self {
        LoadBalancingAlgorithms::RoundRobin
    }
}

pub trait LoadBalancingAlgorithm: Debug {
    fn next_available_backend(
        &mut self,
        backends: &mut Vec<Rc<RefCell<Backend>>>,
    ) -> Option<Rc<RefCell<Backend>>>;
}
