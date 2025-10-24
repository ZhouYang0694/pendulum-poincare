use crate::dynamics::wrap_angle_pi;
use crate::types::*;

pub struct PoincareSampler {
    pub k: usize,
    pub counter: usize,
    pub wrap_to_pi: bool,
}

impl PoincareSampler {
    pub fn new(k: usize, wrap_to_pi: bool) -> Self {
        Self { k, counter: 0, wrap_to_pi }
    }

    pub fn reset(&mut self) {
        self.counter = 0;
    }

    pub fn should_record(&mut self) -> bool {
        self.counter = self.counter.wrapping_add(1);
        self.counter % self.k == 0
    }

    pub fn on_sample(&self, state: &State) -> SamplePoint {
        let theta = if self.wrap_to_pi {
            wrap_angle_pi(state.theta)
        } else {
            state.theta
        };
        SamplePoint { theta, omega: state.omega }
    }
}
