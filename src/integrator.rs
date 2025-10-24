use crate::types::*;
use std::f64::consts::PI;

pub trait Stepper {
    fn step(&mut self, s: State, phys: &PhysicalParams, dt: f64) -> State;
}

pub struct EulerCromer;

pub struct RK4;

impl Stepper for EulerCromer {
    fn step(&mut self, s: State, phys: &PhysicalParams, dt: f64) -> State {
        let domega_dt = -(phys.g / phys.l) * s.theta.sin() - phys.q * s.omega + phys.f_drive * (phys.omega_d * s.t).sin();
        let omega = s.omega + dt * domega_dt;
        let theta = s.theta + dt * omega;
        State { t: s.t + dt, theta, omega }
    }
}

impl Stepper for RK4 {
    fn step(&mut self, s: State, phys: &PhysicalParams, dt: f64) -> State {
        let f = |theta: f64, omega: f64, t: f64| -> (f64, f64) {
            let dtheta = omega;
            let domega = -(phys.g / phys.l) * theta.sin() - phys.q * omega + phys.f_drive * (phys.omega_d * t).sin();
            (dtheta, domega)
        };
        let (k1_theta, k1_omega) = f(s.theta, s.omega, s.t);
        let (k2_theta, k2_omega) = f(s.theta + 0.5 * dt * k1_theta, s.omega + 0.5 * dt * k1_omega, s.t + 0.5 * dt);
        let (k3_theta, k3_omega) = f(s.theta + 0.5 * dt * k2_theta, s.omega + 0.5 * dt * k2_omega, s.t + 0.5 * dt);
        let (k4_theta, k4_omega) = f(s.theta + dt * k3_theta, s.omega + dt * k3_omega, s.t + dt);
        let theta = s.theta + dt * (k1_theta + 2.0 * k2_theta + 2.0 * k3_theta + k4_theta) / 6.0;
        let omega = s.omega + dt * (k1_omega + 2.0 * k2_omega + 2.0 * k3_omega + k4_omega) / 6.0;
        State { t: s.t + dt, theta, omega }
    }
}

pub fn derive_dt_and_k(phys: &PhysicalParams, integ: &IntegratorParams) -> (f64, usize) {
    let p = 2.0 * PI / phys.omega_d;
    match integ.dt_user {
        Some(dt) if dt > 0.0 => {
            let mut k = ((p / dt).round() as i64).max(1) as usize;
            k = snap_even(k);
            let dt_eff = p / k as f64;
            (dt_eff, k)
        }
        _ => {
            let mut k0 = match integ.method {
                IntegratorMethod::RK4 => (p / 0.04).round() as usize,
                IntegratorMethod::EulerCromer => (p / 0.02).round() as usize,
            };
            if k0 == 0 {
                k0 = 1;
            }
            if phys.f_drive >= 1.0 || phys.q <= 0.3 {
                k0 = match integ.method {
                    IntegratorMethod::RK4 => (1.5 * k0 as f64).round() as usize,
                    IntegratorMethod::EulerCromer => (2.0 * k0 as f64).round() as usize,
                };
            }
            k0 = snap_even(k0);
            let dt = p / k0 as f64;
            (dt, k0)
        }
    }
}

pub fn steps_for_warmup(integ: &IntegratorParams, k: usize) -> usize {
    integ.n_periods_warmup.saturating_mul(k)
}

pub fn steps_for_sampling(integ: &IntegratorParams, k: usize) -> usize {
    integ.n_periods_samples.saturating_mul(k)
}

fn snap_even(x: usize) -> usize {
    let lower = if x % 2 == 0 { x } else { x.saturating_sub(1) };
    let higher = lower.checked_add(2).unwrap_or(lower);
    let dist_lower = x.saturating_sub(lower);
    let dist_higher = higher.saturating_sub(x);
    if dist_higher <= dist_lower {
        higher
    } else {
        lower
    }
}
