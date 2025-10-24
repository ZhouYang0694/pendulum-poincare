use crate::integrator::{
	derive_dt_and_k,
	steps_for_sampling,
	steps_for_warmup,
	AdaptiveStepper,
	BulirschStoer,
	EulerCromer,
	RK4,
	RK45,
	Stepper,
};
use crate::sampling::{PoincareSampler, TimeGridSampler};
use crate::settings::drive_period;
use crate::types::*;

pub fn run(spec: &RunSpec) -> Vec<SamplePoint> {
	match spec.integrator.method {
		IntegratorMethod::EulerCromer | IntegratorMethod::RK4 => run_fixed(spec),
		IntegratorMethod::RK45 | IntegratorMethod::BulirschStoer => run_adaptive(spec),
	}
}

fn run_fixed(spec: &RunSpec) -> Vec<SamplePoint> {
	let mut stepper = build_stepper(spec.integrator.method);
	let (dt, k) = derive_dt_and_k(&spec.phys, &spec.integrator);
	let warmup_steps = steps_for_warmup(&spec.integrator, k);
	let sample_steps = steps_for_sampling(&spec.integrator, k);
	let state = State { t: spec.init.t0, theta: spec.init.theta0, omega: spec.init.omega0 };
	let state = integrate_warmup(stepper.as_mut(), state, &spec.phys, dt, warmup_steps);
	let mut sampler = PoincareSampler::new(k, spec.poincare.wrap_to_pi);
	integrate_and_sample(stepper.as_mut(), state, &spec.phys, dt, sample_steps, &mut sampler)
}

fn run_adaptive(spec: &RunSpec) -> Vec<SamplePoint> {
	let period = drive_period(spec.phys.omega_d);
	let mut state = State { t: spec.init.t0, theta: spec.init.theta0, omega: spec.init.omega0 };
	let mut sampler = TimeGridSampler::new(state.t, spec.integrator.n_periods_warmup, period, spec.poincare.wrap_to_pi);
	let mut stepper = build_adaptive(spec.integrator.method);
	let mut dt_init = spec.integrator.dt_init.unwrap_or(period / 400.0);
	let dt_min = spec.integrator.dt_min.unwrap_or(period / 20000.0);
	let dt_max = spec.integrator.dt_max.unwrap_or(period / 20.0);
	let rtol = spec.integrator.rtol.unwrap_or(1e-8);
	let atol = spec.integrator.atol.unwrap_or(1e-10);
	let mut points = Vec::with_capacity(spec.integrator.n_periods_samples);
	dt_init = dt_init.clamp(dt_min, dt_max);
	for _ in 0..spec.integrator.n_periods_samples {
		let target = sampler.target_time();
		let (next_state, used_dt) = stepper.advance_to(state, &spec.phys, target, rtol, atol, dt_init, dt_min, dt_max);
		points.push(sampler.on_sample(&next_state));
		sampler.advance();
		state = next_state;
		dt_init = used_dt.clamp(dt_min, dt_max);
	}
	points
}

pub fn build_stepper(method: IntegratorMethod) -> Box<dyn Stepper> {
	match method {
		IntegratorMethod::EulerCromer => Box::new(EulerCromer),
		IntegratorMethod::RK4 => Box::new(RK4),
		_ => panic!("adaptive method requested from fixed-step builder"),
	}
}

pub fn build_adaptive(method: IntegratorMethod) -> Box<dyn AdaptiveStepper> {
	match method {
		IntegratorMethod::RK45 => Box::new(RK45),
		IntegratorMethod::BulirschStoer => Box::new(BulirschStoer),
		_ => panic!("fixed-step method requested from adaptive builder"),
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
