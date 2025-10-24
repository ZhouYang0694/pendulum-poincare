use crate::integrator::{derive_dt_and_k, steps_for_sampling, steps_for_warmup, EulerCromer, RK4, Stepper};
use crate::sampling::PoincareSampler;
use crate::types::*;

pub fn run(spec: &RunSpec) -> Vec<SamplePoint> {
	let mut stepper = build_stepper(spec.integrator.method);
	let (dt, k) = derive_dt_and_k(&spec.phys, &spec.integrator);
	let warmup_steps = steps_for_warmup(&spec.integrator, k);
	let sample_steps = steps_for_sampling(&spec.integrator, k);
	let state = State { t: spec.init.t0, theta: spec.init.theta0, omega: spec.init.omega0 };
	let state = integrate_warmup(stepper.as_mut(), state, &spec.phys, dt, warmup_steps);
	let mut sampler = PoincareSampler::new(k, spec.poincare.wrap_to_pi);
	integrate_and_sample(stepper.as_mut(), state, &spec.phys, dt, sample_steps, &mut sampler)
}

pub fn build_stepper(method: IntegratorMethod) -> Box<dyn Stepper> {
	match method {
		IntegratorMethod::EulerCromer => Box::new(EulerCromer),
		IntegratorMethod::RK4 => Box::new(RK4),
	}
}

pub fn integrate_warmup(stepper: &mut dyn Stepper, state: State, phys: &PhysicalParams, dt: f64, steps: usize) -> State {
	let mut current = state;
	for _ in 0..steps {
		current = stepper.step(current, phys, dt);
	}
	current
}

pub fn integrate_and_sample(stepper: &mut dyn Stepper, state: State, phys: &PhysicalParams, dt: f64, steps: usize, sampler: &mut PoincareSampler) -> Vec<SamplePoint> {
	let reserve = if sampler.k == 0 { 0 } else { steps / sampler.k + 1 };
	let mut points = Vec::with_capacity(reserve);
	let mut current = state;
	for _ in 0..steps {
		current = stepper.step(current, phys, dt);
		if sampler.should_record() {
			points.push(sampler.on_sample(&current));
		}
	}
	points
}
