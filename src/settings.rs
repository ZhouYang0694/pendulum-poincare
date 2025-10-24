use crate::types::*;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn load_run_spec(path: &str) -> RunSpec {
	let contents = fs::read_to_string(Path::new(path)).expect("failed to read run spec file");
	let mut value: Value = serde_json::from_str(&contents).expect("failed to parse run spec json");
	if let Value::Object(ref mut map) = value {
		let entry = map
			.entry("poincare".to_string())
			.or_insert_with(|| Value::Object(serde_json::Map::new()));
		match entry {
			Value::Object(ref mut poincare_map) => {
				poincare_map
					.entry("wrap_to_pi".to_string())
					.or_insert(Value::Bool(true));
			}
			other => {
				let mut poincare_map = serde_json::Map::new();
				poincare_map.insert("wrap_to_pi".to_string(), Value::Bool(true));
				*other = Value::Object(poincare_map);
			}
		}
	} else {
		panic!("run spec root must be a JSON object");
	}
	let mut spec: RunSpec = serde_json::from_value(value).expect("failed to deserialize run spec");
	coerce_fixed_theta_range(&mut spec.plot);
	derive_outputs(&mut spec);
	validate_run_spec(&spec);
	spec
}

pub fn coerce_fixed_theta_range(view: &mut PlotView) {
	view.theta_min = -std::f64::consts::PI;
	view.theta_max = std::f64::consts::PI;
}

pub fn validate_run_spec(spec: &RunSpec) {
	assert!(spec.phys.g > 0.0, "gravity must be positive");
	assert!(spec.phys.l > 0.0, "pendulum length must be positive");
	assert!(spec.phys.q >= 0.0, "damping must be non-negative");
	assert!(spec.phys.omega_d > 0.0, "drive frequency must be positive");
	assert!(spec.integrator.n_periods_warmup >= 0, "warmup periods must be non-negative");
	assert!(spec.integrator.n_periods_samples > 0, "sample periods must be positive");
	assert!(spec.plot.width_px >= 200, "plot width must be at least 200");
	assert!(spec.plot.height_px >= 200, "plot height must be at least 200");
	assert!(!spec.output.out_base.trim().is_empty(), "output base cannot be empty");
}

pub fn derive_outputs(spec: &mut RunSpec) {
	if spec.output.out_base.trim().is_empty() {
		spec.output.out_base = "poincare".to_string();
	}
}

pub fn drive_period(omega_d: f64) -> f64 {
	2.0 * std::f64::consts::PI / omega_d
}
