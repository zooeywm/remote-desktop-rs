use serde::Deserialize;
use tracing_appender::rolling::Rotation;

/// Telemetry system configuration
#[derive(Debug, Deserialize)]
pub struct TelemetryConfig {
	#[serde(default = "TelemetryConfig::default_app_name")]
	pub app_name: String,
	/// Whether to turn on
	#[serde(default = "CommonLogConfig::default_enable")]
	pub enable:   bool,

	/// Console output configuration
	#[serde(default)]
	pub console: ConsoleConfig,

	/// Remote output configuration
	#[serde(default)]
	pub remote: RemoteConfig,

	/// File output configuration
	#[serde(default)]
	pub file: FileConfig,

	#[serde(default = "TelemetryConfig::default_timezone")]
	pub timezone: i8,
}

/// Console output configuration
#[derive(Debug, Default, Deserialize)]
pub struct ConsoleConfig(pub CommonLogConfig);

#[derive(Debug, Deserialize)]
pub struct CommonLogConfig {
	/// Whether to turn on
	#[serde(default = "CommonLogConfig::default_enable")]
	pub enable: bool,

	/// Debug mode: log content is more detailed
	#[serde(default)]
	pub verbose: bool,

	/// Custom filter rules
	#[serde(default)]
	pub filter: String,

	/// Custom filtering rule environment variables
	#[serde(default)]
	pub filter_env: String,
}

/// File output configuration
#[derive(Debug, Deserialize)]
pub struct FileConfig {
	#[serde(flatten)]
	pub common: CommonLogConfig,

	/// Custom log folder location (default `./logs`)
	#[serde(default = "FileConfig::default_path")]
	pub path: String,

	/// Customize log file name, or rolling write prefix (default `prefix.log`)
	#[serde(default = "FileConfig::default_filename")]
	pub prefix: String,

	#[serde(flatten)]
	pub r#type: FileLogType,
}

/// Call tracing configuration
#[derive(Debug, Default, Deserialize)]
pub struct RemoteConfig {
	/// Enable call tracing
	#[serde(default)]
	pub enable: bool,

	/// Remote collector address
	#[serde(default)]
	pub collector_endpoint: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FileLogType {
	Rolling {
		/// The writing duration of rolling creation files. The default is `Never`,
		/// which means writing of rolling creation files is prohibited.
		#[serde(default)]
		rolling_time: LogRotation,
	},
	#[default]
	New,
}

#[derive(Debug, Default, Clone, Copy, Deserialize)]
#[serde(from = "String", rename_all = "snake_case")]
pub enum LogRotation {
	Daily,
	Hourly,
	Minutely,
	#[default]
	Never,
}

impl From<String> for LogRotation {
	fn from(s: String) -> Self {
		match s.to_lowercase().as_str() {
			"daily" => Self::Daily,
			"hourly" => Self::Hourly,
			"minutely" => Self::Minutely,
			_ => Self::Never,
		}
	}
}

impl From<LogRotation> for Rotation {
	fn from(val: LogRotation) -> Self {
		match val {
			LogRotation::Daily => Rotation::DAILY,
			LogRotation::Hourly => Rotation::HOURLY,
			LogRotation::Minutely => Rotation::MINUTELY,
			LogRotation::Never => Rotation::NEVER,
		}
	}
}

impl CommonLogConfig {
	fn default_enable() -> bool { true }
}

impl FileConfig {
	fn default_path() -> String { String::from("./logs") }

	fn default_filename() -> String { String::from("prefix.log") }
}

impl Default for TelemetryConfig {
	fn default() -> Self {
		Self {
			app_name: Default::default(),
			enable:   CommonLogConfig::default_enable(),
			console:  Default::default(),
			remote:   Default::default(),
			file:     Default::default(),
			timezone: Default::default(),
		}
	}
}

impl TelemetryConfig {
	fn default_app_name() -> String { String::from("remote-desk-rs") }

	fn default_timezone() -> i8 { 0 }
}

impl Default for CommonLogConfig {
	fn default() -> Self {
		Self {
			enable:     Self::default_enable(),
			verbose:    Default::default(),
			filter:     Default::default(),
			filter_env: Default::default(),
		}
	}
}

impl Default for FileConfig {
	fn default() -> Self {
		Self {
			common: Default::default(),
			path:   Self::default_path(),
			prefix: Self::default_filename(),
			r#type: Default::default(),
		}
	}
}
