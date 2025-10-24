pub enum IntegratorMethod {
    EulerCromer,
    RK4,
}

pub struct PhysicalParams {
    pub g: f64,
    pub l: f64,
    pub q: f64,
    pub f_drive: f64,
    pub omega_d: f64,
}

pub struct IntegratorParams {
    pub method: IntegratorMethod,
    pub dt_user: Option<f64>,
    pub n_periods_warmup: usize,
    pub n_periods_samples: usize,
}

pub struct InitialState {
    pub theta0: f64,
    pub omega0: f64,
    pub t0: f64,
}

pub struct PoincareConfig {
    pub wrap_to_pi: bool,
}

pub struct OutputConfig {
    pub out_base: String,
}

pub struct PlotView {
    pub theta_min: f64,
    pub theta_max: f64,
    pub omega_min: f64,
    pub omega_max: f64,
    pub width_px: u32,
    pub height_px: u32,
    pub title: String,
}

pub struct RunSpec {
    pub phys: PhysicalParams,
    pub integrator: IntegratorParams,
    pub init: InitialState,
    pub poincare: PoincareConfig,
    pub plot: PlotView,
    pub output: OutputConfig,
}

pub struct State {
    pub t: f64,
    pub theta: f64,
    pub omega: f64,
}

pub struct SamplePoint {
    pub theta: f64,
    pub omega: f64,
}
