use crate::types::*;

pub struct PoincareSampler {
    pub k: usize,
    pub counter: usize,
    pub wrap_to_pi: bool,
}

impl PoincareSampler {
    pub fn new(k: usize, wrap_to_pi: bool) -> Self {}

    pub fn should_record(&mut self) -> bool {}

    pub fn on_sample(&self, state: &State) -> SamplePoint {}
}
