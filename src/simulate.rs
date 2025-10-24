use crate::types::*;
use crate::integrator::*;
use crate::sampling::*;

pub fn run(spec: &RunSpec) -> Vec<SamplePoint> {}

pub fn build_stepper(method: IntegratorMethod) -> Box<dyn Stepper> {}

pub fn integrate_warmup(stepper: &mut dyn Stepper, state: State, phys: &PhysicalParams, dt: f64, steps: usize) -> State {}

pub fn integrate_and_sample(stepper: &mut dyn Stepper, state: State, phys: &PhysicalParams, dt: f64, steps: usize, sampler: &mut PoincareSampler) -> Vec<SamplePoint> {}
