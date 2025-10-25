use crate::types::*;
use std::f64::{consts::PI, EPSILON};

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

pub trait AdaptiveStepper {
    fn advance_to(
        &mut self,
        s: State,
        phys: &PhysicalParams,
        t_target: f64,
        rtol: f64,
        atol: f64,
        dt_init: f64,
        dt_min: f64,
        dt_max: f64,
    ) -> (State, f64);
}

pub struct RK45;

impl AdaptiveStepper for RK45 {
    fn advance_to(
        &mut self,
        s: State,
        phys: &PhysicalParams,
        t_target: f64,
        rtol: f64,
        atol: f64,
        dt_init: f64,
        dt_min: f64,
        dt_max: f64,
    ) -> (State, f64) {
        if t_target <= s.t + EPSILON {
            return (State { t: t_target, ..s }, dt_init.max(dt_min));
        }
        let safety = 0.9;
        let min_factor = 0.2;
        let max_factor = 5.0;
        let mut state = s;
        let mut h = dt_init.clamp(dt_min, dt_max);
        if h <= 0.0 {
            h = dt_min.max(t_target - state.t);
        }
        let mut last_h = h;
        while t_target - state.t > EPSILON {
            let remaining = t_target - state.t;
            let mut h_trial = h.min(dt_max);
            if h_trial > remaining {
                h_trial = remaining;
            }
            if h_trial < dt_min && remaining > dt_min {
                h_trial = dt_min;
            }
            if h_trial <= 0.0 {
                h_trial = remaining;
            }
            loop {
                let (k1_theta, k1_omega) = rhs(state.theta, state.omega, state.t, phys);
                let y2_theta = state.theta + h_trial * (1.0 / 5.0) * k1_theta;
                let y2_omega = state.omega + h_trial * (1.0 / 5.0) * k1_omega;
                let (k2_theta, k2_omega) = rhs(y2_theta, y2_omega, state.t + h_trial * (1.0 / 5.0), phys);
                let y3_theta = state.theta
                    + h_trial
                        * ((3.0 / 40.0) * k1_theta + (9.0 / 40.0) * k2_theta);
                let y3_omega = state.omega
                    + h_trial
                        * ((3.0 / 40.0) * k1_omega + (9.0 / 40.0) * k2_omega);
                let (k3_theta, k3_omega) = rhs(y3_theta, y3_omega, state.t + h_trial * (3.0 / 10.0), phys);
                let y4_theta = state.theta
                    + h_trial
                        * ((44.0 / 45.0) * k1_theta
                            + (-56.0 / 15.0) * k2_theta
                            + (32.0 / 9.0) * k3_theta);
                let y4_omega = state.omega
                    + h_trial
                        * ((44.0 / 45.0) * k1_omega
                            + (-56.0 / 15.0) * k2_omega
                            + (32.0 / 9.0) * k3_omega);
                let (k4_theta, k4_omega) = rhs(y4_theta, y4_omega, state.t + h_trial * (4.0 / 5.0), phys);
                let y5_theta = state.theta
                    + h_trial
                        * ((19372.0 / 6561.0) * k1_theta
                            + (-25360.0 / 2187.0) * k2_theta
                            + (64448.0 / 6561.0) * k3_theta
                            + (-212.0 / 729.0) * k4_theta);
                let y5_omega = state.omega
                    + h_trial
                        * ((19372.0 / 6561.0) * k1_omega
                            + (-25360.0 / 2187.0) * k2_omega
                            + (64448.0 / 6561.0) * k3_omega
                            + (-212.0 / 729.0) * k4_omega);
                let (k5_theta, k5_omega) = rhs(y5_theta, y5_omega, state.t + h_trial * (8.0 / 9.0), phys);
                let y6_theta = state.theta
                    + h_trial
                        * ((9017.0 / 3168.0) * k1_theta
                            + (-355.0 / 33.0) * k2_theta
                            + (46732.0 / 5247.0) * k3_theta
                            + (49.0 / 176.0) * k4_theta
                            + (-5103.0 / 18656.0) * k5_theta);
                let y6_omega = state.omega
                    + h_trial
                        * ((9017.0 / 3168.0) * k1_omega
                            + (-355.0 / 33.0) * k2_omega
                            + (46732.0 / 5247.0) * k3_omega
                            + (49.0 / 176.0) * k4_omega
                            + (-5103.0 / 18656.0) * k5_omega);
                let (k6_theta, k6_omega) = rhs(y6_theta, y6_omega, state.t + h_trial, phys);
                let y7_theta = state.theta
                    + h_trial
                        * ((35.0 / 384.0) * k1_theta
                            + (500.0 / 1113.0) * k3_theta
                            + (125.0 / 192.0) * k4_theta
                            + (-2187.0 / 6784.0) * k5_theta
                            + (11.0 / 84.0) * k6_theta);
                let y7_omega = state.omega
                    + h_trial
                        * ((35.0 / 384.0) * k1_omega
                            + (500.0 / 1113.0) * k3_omega
                            + (125.0 / 192.0) * k4_omega
                            + (-2187.0 / 6784.0) * k5_omega
                            + (11.0 / 84.0) * k6_omega);
                let (k7_theta, k7_omega) = rhs(y7_theta, y7_omega, state.t + h_trial, phys);
                let theta5 = state.theta
                    + h_trial
                        * ((35.0 / 384.0) * k1_theta
                            + (500.0 / 1113.0) * k3_theta
                            + (125.0 / 192.0) * k4_theta
                            + (-2187.0 / 6784.0) * k5_theta
                            + (11.0 / 84.0) * k6_theta);
                let omega5 = state.omega
                    + h_trial
                        * ((35.0 / 384.0) * k1_omega
                            + (500.0 / 1113.0) * k3_omega
                            + (125.0 / 192.0) * k4_omega
                            + (-2187.0 / 6784.0) * k5_omega
                            + (11.0 / 84.0) * k6_omega);
                let theta4 = state.theta
                    + h_trial
                        * ((5179.0 / 57600.0) * k1_theta
                            + (7571.0 / 16695.0) * k3_theta
                            + (393.0 / 640.0) * k4_theta
                            + (-92097.0 / 339200.0) * k5_theta
                            + (187.0 / 2100.0) * k6_theta
                            + (1.0 / 40.0) * k7_theta);
                let omega4 = state.omega
                    + h_trial
                        * ((5179.0 / 57600.0) * k1_omega
                            + (7571.0 / 16695.0) * k3_omega
                            + (393.0 / 640.0) * k4_omega
                            + (-92097.0 / 339200.0) * k5_omega
                            + (187.0 / 2100.0) * k6_omega
                            + (1.0 / 40.0) * k7_omega);
                let err = error_norm(
                    theta5 - theta4,
                    omega5 - omega4,
                    theta5,
                    omega5,
                    state.theta,
                    state.omega,
                    rtol,
                    atol,
                );
                if err <= 1.0 || h_trial <= dt_min {
                    state = State { t: state.t + h_trial, theta: theta5, omega: omega5 };
                    if t_target - state.t <= EPSILON {
                        state.t = t_target;
                    }
                    last_h = h_trial;
                    let factor = if err <= 1e-12 {
                        max_factor
                    } else {
                        (safety * err.powf(-0.2)).clamp(min_factor, max_factor)
                    };
                    h = (h_trial * factor).clamp(dt_min, dt_max);
                    if h > remaining {
                        h = remaining;
                    }
                    break;
                } else {
                    let factor = (safety * err.powf(-0.2)).clamp(min_factor, 1.0);
                    let mut new_h = h_trial * factor;
                    if new_h < dt_min && remaining > dt_min {
                        new_h = dt_min;
                    }
                    if new_h <= EPSILON {
                        new_h = remaining;
                    }
                    h_trial = new_h.min(remaining);
                }
            }
        }
        state.t = t_target;
        (state, last_h)
    }
}

pub struct BulirschStoer;

impl BulirschStoer {
    fn modified_midpoint(state: &State, phys: &PhysicalParams, h: f64, n: usize) -> (f64, f64) {
        let h_n = h / n as f64;
        let mut y_prev_theta = state.theta;
        let mut y_prev_omega = state.omega;
        let (dtheta, domega) = rhs(state.theta, state.omega, state.t, phys);
        let mut y_curr_theta = state.theta + h_n * dtheta;
        let mut y_curr_omega = state.omega + h_n * domega;
        let mut t = state.t + h_n;
        for _ in 1..n {
            let (dtheta_curr, domega_curr) = rhs(y_curr_theta, y_curr_omega, t, phys);
            let next_theta = y_prev_theta + 2.0 * h_n * dtheta_curr;
            let next_omega = y_prev_omega + 2.0 * h_n * domega_curr;
            y_prev_theta = y_curr_theta;
            y_prev_omega = y_curr_omega;
            y_curr_theta = next_theta;
            y_curr_omega = next_omega;
            t += h_n;
        }
        let (dtheta_end, domega_end) = rhs(y_curr_theta, y_curr_omega, state.t + h, phys);
        let theta = 0.5 * (y_prev_theta + y_curr_theta + h_n * dtheta_end);
        let omega = 0.5 * (y_prev_omega + y_curr_omega + h_n * domega_end);
        (theta, omega)
    }
}

impl AdaptiveStepper for BulirschStoer {
    fn advance_to(
        &mut self,
        s: State,
        phys: &PhysicalParams,
        t_target: f64,
        rtol: f64,
        atol: f64,
        dt_init: f64,
        dt_min: f64,
        dt_max: f64,
    ) -> (State, f64) {
        if t_target <= s.t + EPSILON {
            return (State { t: t_target, ..s }, dt_init.max(dt_min));
        }
        let safety = 0.9;
        let min_factor = 0.2;
        let max_factor = 5.0;
        let mut state = s;
        let mut h = dt_init.clamp(dt_min, dt_max);
        if h <= 0.0 {
            h = dt_min.max(t_target - state.t);
        }
        let mut last_h = h;
        let n_seq = [2usize, 4, 6, 8, 10, 12];
        while t_target - state.t > EPSILON {
            let remaining = t_target - state.t;
            let mut h_trial = h.min(dt_max);
            if h_trial > remaining {
                h_trial = remaining;
            }
            if h_trial < dt_min && remaining > dt_min {
                h_trial = dt_min;
            }
            if h_trial <= 0.0 {
                h_trial = remaining;
            }
            loop {
                let mut table: Vec<Vec<(f64, f64)>> = Vec::new();
                let mut accepted = false;
                let mut err = 0.0;
                let mut best = (state.theta, state.omega);
                for (i, &n) in n_seq.iter().enumerate() {
                    let approx = BulirschStoer::modified_midpoint(&state, phys, h_trial, n);
                    if table.len() <= i {
                        table.push(Vec::new());
                    }
                    table[i].push(approx);
                    for k in 1..=i {
                        let ratio = (n_seq[i] as f64 / n_seq[i - k] as f64).powi(2) - 1.0;
                        let prev = table[i][k - 1];
                        let prev_row = table[i - 1][k - 1];
                        let extrap = (
                            prev.0 + (prev.0 - prev_row.0) / ratio,
                            prev.1 + (prev.1 - prev_row.1) / ratio,
                        );
                        table[i].push(extrap);
                    }
                    if i > 0 {
                        let current = table[i][i];
                        let prev_col = table[i][i - 1];
                        err = error_norm(
                            current.0 - prev_col.0,
                            current.1 - prev_col.1,
                            current.0,
                            current.1,
                            state.theta,
                            state.omega,
                            rtol,
                            atol,
                        );
                        if err <= 1.0 || h_trial <= dt_min {
                            accepted = true;
                            best = current;
                            break;
                        }
                    }
                }
                if accepted {
                    state = State { t: state.t + h_trial, theta: best.0, omega: best.1 };
                    if t_target - state.t <= EPSILON {
                        state.t = t_target;
                    }
                    last_h = h_trial;
                    let factor = if err <= 1e-12 {
                        max_factor
                    } else {
                        (safety * err.powf(-0.2)).clamp(min_factor, max_factor)
                    };
                    h = (h_trial * factor).clamp(dt_min, dt_max);
                    if h > remaining {
                        h = remaining;
                    }
                    break;
                } else {
                    let factor = if err <= 1e-12 {
                        min_factor
                    } else {
                        (safety * err.powf(-0.2)).clamp(min_factor, 1.0)
                    };
                    let mut new_h = h_trial * factor;
                    if new_h < dt_min && remaining > dt_min {
                        new_h = dt_min;
                    }
                    if new_h <= EPSILON {
                        new_h = remaining;
                    }
                    h_trial = new_h.min(remaining);
                    continue;
                }
            }
        }
        state.t = t_target;
        (state, last_h)
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
                IntegratorMethod::RK4 | IntegratorMethod::RK45 | IntegratorMethod::BulirschStoer => (p / 0.04).round() as usize,
                IntegratorMethod::EulerCromer => (p / 0.02).round() as usize,
            };
            if k0 == 0 {
                k0 = 1;
            }
            if phys.f_drive >= 1.0 || phys.q <= 0.3 {
                k0 = match integ.method {
                    IntegratorMethod::RK4 | IntegratorMethod::RK45 | IntegratorMethod::BulirschStoer => (1.5 * k0 as f64).round() as usize,
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

fn rhs(theta: f64, omega: f64, t: f64, phys: &PhysicalParams) -> (f64, f64) {
    let dtheta = omega;
    let domega = -(phys.g / phys.l) * theta.sin() - phys.q * omega + phys.f_drive * (phys.omega_d * t).sin();
    (dtheta, domega)
}

fn error_norm(
    diff_theta: f64,
    diff_omega: f64,
    new_theta: f64,
    new_omega: f64,
    old_theta: f64,
    old_omega: f64,
    rtol: f64,
    atol: f64,
) -> f64 {
    let scale_theta = atol + rtol * new_theta.abs().max(old_theta.abs());
    let scale_omega = atol + rtol * new_omega.abs().max(old_omega.abs());
    (diff_theta / scale_theta).abs().max((diff_omega / scale_omega).abs())
}
