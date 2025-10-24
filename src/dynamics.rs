use crate::types::*;
use std::f64::consts::PI;

pub fn pendulum_rhs(state: &State, phys: &PhysicalParams) -> (f64, f64) {
	let dtheta_dt = state.omega;
	let domega_dt = -(phys.g / phys.l) * state.theta.sin() - phys.q * state.omega + phys.f_drive * (phys.omega_d * state.t).sin();
	(dtheta_dt, domega_dt)
}

pub fn wrap_angle_pi(theta: f64) -> f64 {
	let two_pi = 2.0 * PI;
	let wrapped = theta.rem_euclid(two_pi);
	if wrapped <= PI {
		wrapped
	} else {
		wrapped - two_pi
	}
}
