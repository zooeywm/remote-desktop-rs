pub mod config;

use std::{io::IsTerminal, path::Path, sync::OnceLock};

use opentelemetry::trace::{TraceResult, TracerProvider};
use opentelemetry_otlp::WithExportConfig;
use rdrs_tools::error::Result;
use time::UtcOffset;
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::{Layer as _, Registry, filter::{EnvFilter, LevelFilter}, fmt::{time::OffsetTime, writer::BoxMakeWriter}, layer::SubscriberExt, reload::{self, Handle}, util::SubscriberInitExt};

pub use self::config::*;

const DEFAULT_LEVEL: LevelFilter = LevelFilter::INFO;
type ReloadHandle = Handle<EnvFilter, Registry>;

static CONSOLE_RELOAD_HANDLE: OnceLock<ReloadHandle> = OnceLock::new();

/// Configuration log
pub fn init_telemetry(config: &TelemetryConfig) -> Result<()> {
	let TelemetryConfig { ref app_name, enable, ref console, ref remote, ref file, timezone } =
		*config;

	if !enable {
		return Ok(());
	}

	let get_filter = |filter_env: &str, filter: &str| -> EnvFilter {
		let filter_builder = EnvFilter::builder().with_default_directive(DEFAULT_LEVEL.into());
		if !filter_env.is_empty() {
			filter_builder.with_env_var(filter_env).from_env_lossy()
		} else {
			filter_builder.parse_lossy(filter)
		}
	};

	let offset = UtcOffset::from_hms(timezone, 0, 0).unwrap();
	let format = time::format_description::parse(
		"[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]+[offset_hour]",
	)?;
	let timer = OffsetTime::new(offset, format);

	// Console config
	let ConsoleConfig(CommonLogConfig { enable, verbose, ref filter, ref filter_env }) = *console;
	let console = enable.then(|| {
		let filter = get_filter(filter_env, filter);
		let (filter, reload_handle) = reload::Layer::new(filter);
		let _ = CONSOLE_RELOAD_HANDLE.set(reload_handle);
		tracing_subscriber::fmt::layer()
			.with_file(verbose)
			.with_line_number(verbose)
			.with_thread_ids(verbose)
			.with_target(true)
			.with_timer(timer.clone())
			.with_filter(filter)
	});

	// File config
	let FileConfig {
		common: CommonLogConfig { enable, verbose, ref filter, ref filter_env },
		ref path,
		ref prefix,
		ref r#type,
	} = *file;
	let file = enable
		.then(|| {
			let filter = get_filter(filter_env, filter);

			let file_writer = match *r#type {
				FileLogType::Rolling { rolling_time } => {
					let file_appender = RollingFileAppender::new(rolling_time.into(), path, prefix);
					BoxMakeWriter::new(file_appender)
				}
				FileLogType::New => {
					std::fs::create_dir_all(path)?;
					let path = Path::new(path)
						.join(format!("{prefix}-{}.log", chrono::Local::now().format("%Y%m%d-%H%M%S")));
					BoxMakeWriter::new(std::sync::Mutex::new(std::fs::File::create(path)?))
				}
			};

			let layer = tracing_subscriber::fmt::layer()
				.with_ansi(std::io::stderr().is_terminal())
				.with_writer(file_writer)
				.with_file(true)
				.with_line_number(true)
				.with_thread_ids(verbose)
				.with_target(true)
				.with_timer(timer)
				.with_filter(filter);

			Result::Ok(layer)
		})
		.transpose()?;

	let RemoteConfig { enable, ref collector_endpoint } = *remote;
	let remote = enable
		.then(|| {
			let mut exporter = opentelemetry_otlp::new_exporter().tonic();
			if !remote.collector_endpoint.is_empty() {
				exporter = exporter.with_endpoint(collector_endpoint);
			}

			let tracer = opentelemetry_otlp::new_pipeline()
				.tracing()
				.with_exporter(exporter)
				.install_batch(opentelemetry_sdk::runtime::Tokio)?
				.tracer(app_name.clone());

			TraceResult::Ok(tracing_opentelemetry::layer().with_tracer(tracer))
		})
		.transpose()?;

	Registry::default().with(console).with(file).with(remote).try_init()?;
	Ok(())
}

/// Reload console subscriber
pub fn reload_console_envfilter(filter: &str) -> Result<()> {
	let new_filter =
		EnvFilter::builder().with_default_directive(DEFAULT_LEVEL.into()).parse_lossy(filter);
	if let Some(handle) = CONSOLE_RELOAD_HANDLE.get() {
		handle.modify(|filter| *filter = new_filter)?;
	}

	Ok(())
}
