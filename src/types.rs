use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum IntegratorMethod {
    EulerCromer,
    RK4
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PhysicalParams {
    pub g: f64,
    pub l: f64,
    pub q: f64,
    pub f_drive: f64,
    pub omega_d: f64
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntegratorParams {
    pub method: IntegratorMethod,
    pub dt_user: Option<f64>,
    pub n_periods_warmup: usize,
    pub n_periods_samples: usize
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct InitialState {
    pub theta0: f64,
    pub omega0: f64,
    pub t0: f64
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PoincareConfig {
    pub wrap_to_pi: bool
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputConfig {
    pub out_base: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlotView {
    pub theta_min: f64,
    pub theta_max: f64,
    pub width_px: u32,
    pub height_px: u32,
    pub title: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunSpec {
    pub phys: PhysicalParams,
    pub integrator: IntegratorParams,
    pub init: InitialState,
    pub poincare: PoincareConfig,
    pub plot: PlotView,
    pub output: OutputConfig
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct State {
    pub t: f64,
    pub theta: f64,
    pub omega: f64
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SamplePoint {
    pub theta: f64,
    pub omega: f64
}