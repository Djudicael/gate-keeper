use std::{cell::RefCell, rc::Rc};

use rand::{distributions::WeightedIndex, prelude::Distribution, seq::SliceRandom, thread_rng};

use crate::proxy::{backend::Backend, load_balancing_algorithm::LoadBalancingAlgorithm};

#[derive(Debug)]
pub struct Random;

impl LoadBalancingAlgorithm for Random {
    fn next_available_backend(
        &mut self,
        backends: &mut Vec<Rc<RefCell<Backend>>>,
    ) -> Option<Rc<RefCell<Backend>>> {
        let mut rng = thread_rng();
        let weights: Vec<u8> = backends
            .iter()
            .map(|b| {
                b.borrow()
                    .load_balancing_parameters
                    .as_ref()
                    .map(|p| p.weight)
                    .unwrap_or(100)
            })
            .collect();

        if let Ok(dist) = WeightedIndex::new(&weights) {
            let index = dist.sample(&mut rng);
            backends.get(index).cloned()
        } else {
            (*backends)
                .choose(&mut rng)
                .map(|backend| (*backend).clone())
        }
    }
}
