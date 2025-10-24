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

pub struct TimeGridSampler {
    pub t_next: f64,
    pub period: f64,
    pub wrap_to_pi: bool,
}

impl TimeGridSampler {
    pub fn new(t0: f64, periods_to_skip: usize, period: f64, wrap_to_pi: bool) -> Self {
        let start = t0 + (periods_to_skip as f64 + 1.0) * period;
        Self { t_next: start, period, wrap_to_pi }
    }

    pub fn target_time(&self) -> f64 {
        self.t_next
    }

    pub fn advance(&mut self) {
        self.t_next += self.period;
    }

    pub fn on_sample(&self, state: &State) -> SamplePoint {
        let theta = if self.wrap_to_pi { wrap_angle_pi(state.theta) } else { state.theta };
        SamplePoint { theta, omega: state.omega }
    }
}
