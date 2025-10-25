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
	if let Some(size) = spec.plot.marker_size {
		if size < 1 {
			spec.plot.marker_size = None;
		}
	}
	if spec.plot.title_font_px.unwrap_or(0) < 8 {
		spec.plot.title_font_px = Some(64);
	}
	if spec.plot.axis_label_font_px.unwrap_or(0) < 8 {
		spec.plot.axis_label_font_px = Some(36);
	}
	if spec.plot.tick_font_px.unwrap_or(0) < 6 {
		spec.plot.tick_font_px = Some(28);
	}
	let period = drive_period(spec.phys.omega_d);
	if spec.integrator.rtol.unwrap_or(0.0) <= 0.0 {
		spec.integrator.rtol = Some(1e-8);
	}
	if spec.integrator.atol.unwrap_or(0.0) <= 0.0 {
		spec.integrator.atol = Some(1e-10);
	}
	if spec.integrator.dt_init.unwrap_or(0.0) <= 0.0 {
		spec.integrator.dt_init = Some(period / 400.0);
	}
	if spec.integrator.dt_min.unwrap_or(0.0) <= 0.0 {
		spec.integrator.dt_min = Some(period / 20000.0);
	}
	if spec.integrator.dt_max.unwrap_or(0.0) <= 0.0 {
		spec.integrator.dt_max = Some(period / 20.0);
	}
	derive_outputs(&mut spec);
	validate_run_spec(&spec);
	spec
}

pub fn validate_run_spec(spec: &RunSpec) {
	assert!(spec.phys.g > 0.0, "gravity must be positive");
	assert!(spec.phys.l > 0.0, "pendulum length must be positive");
	assert!(spec.phys.q >= 0.0, "damping must be non-negative");
	assert!(spec.phys.omega_d > 0.0, "drive frequency must be positive");
	assert!(spec.integrator.n_periods_warmup >= 0, "warmup periods must be non-negative");
	assert!(spec.integrator.n_periods_samples > 0, "sample periods must be positive");
	assert!(spec.plot.side_px >= 200, "plot side length must be at least 200");
	if let Some(size) = spec.plot.title_font_px {
		assert!(size >= 8, "title font size must be at least 8");
	}
	if let Some(size) = spec.plot.axis_label_font_px {
		assert!(size >= 8, "axis label font size must be at least 8");
	}
	if let Some(size) = spec.plot.tick_font_px {
		assert!(size >= 6, "tick font size must be at least 6");
	}
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
