use crate::types::*;

pub trait Stepper {
    fn step(&mut self, s: State, phys: &PhysicalParams, dt: f64) -> State;
}

pub struct EulerCromer;

pub struct RK4;

impl Stepper for EulerCromer {
    fn step(&mut self, s: State, phys: &PhysicalParams, dt: f64) -> State {}
}

impl Stepper for RK4 {
    fn step(&mut self, s: State, phys: &PhysicalParams, dt: f64) -> State {}
}

pub fn derive_dt_and_k(phys: &PhysicalParams, integ: &IntegratorParams) -> (f64, usize) {}

pub fn steps_for_warmup(integ: &IntegratorParams, k: usize) -> usize {}

pub fn steps_for_sampling(integ: &IntegratorParams, k: usize) -> usize {}
