use std::path::Path;

use config::{Config, FileFormat};
use remote_desk_core::error::Result;
use serde::Deserialize;

pub fn build_config() -> Result<Config> {
	let args: Vec<String> = std::env::args().collect();
	let mut config = Config::builder()
		.add_source(config::File::with_name("config").required(false).format(FileFormat::Toml));
	for arg in args {
		if arg.ends_with("toml") {
			config = config.add_source(
				config::File::from(Path::new(arg.as_str())).format(FileFormat::Toml).required(false),
			)
		}
	}
	config = config.add_source(
		config::Environment::with_prefix("RDRS")
			.separator("__")
			.try_parsing(true)
			.list_separator(";")
			.with_list_parse_key("common.extends_array"),
	);
	Ok(config.build()?)
}

/// All dependences in `Container` initialized with the same one config, with
/// `extends` field for test or something else
#[derive(Debug, Default, Deserialize)]
pub struct CommonConfig {
	#[cfg(feature = "telemetry")]
	#[serde(default)]
	pub telemetry:     crate::telemetry::TelemetryConfig,
	/// Extend fields
	#[serde(default)]
	pub extends:       Option<config::Value>,
	/// Extend Sstring array
	#[serde(default)]
	pub extends_array: Vec<String>,
}
