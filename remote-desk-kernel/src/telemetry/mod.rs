pub mod config;

use opentelemetry::trace::{TraceResult, TracerProvider};
use opentelemetry_otlp::WithExportConfig;
use remote_desk_core::error::Result;
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::{filter::{EnvFilter, LevelFilter}, layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry};

pub use self::config::*;

const DEFAULT_LEVEL: LevelFilter = LevelFilter::INFO;

/// Configuration log
pub fn init_telemetry(config: &TelemetryConfig) -> Result<()> {
	if !config.enable {
		return Ok(());
	}

	let console = &config.console.0;
	let console = console.enable.then(|| {
		let filter = EnvFilter::builder().with_default_directive(DEFAULT_LEVEL.into());
		let filter = if !console.filter_env.is_empty() {
			filter.with_env_var(&console.filter_env).from_env_lossy()
		} else {
			filter.with_default_directive(DEFAULT_LEVEL.into()).parse_lossy(&console.filter)
		};

		tracing_subscriber::fmt::layer()
			.with_file(console.verbose)
			.with_line_number(console.verbose)
			.with_thread_ids(console.verbose)
			.with_target(true)
			.with_filter(filter)
	});

	let file = &config.file;
	let file = file.common.enable.then(|| {
		let file_appender =
			RollingFileAppender::new(file.rolling_time.into(), &file.path, &file.prefix);

		let filter = EnvFilter::builder().with_default_directive(DEFAULT_LEVEL.into());
		let filter = if !file.common.filter_env.is_empty() {
			filter.with_env_var(&file.common.filter_env).from_env_lossy()
		} else {
			filter.parse_lossy(&file.common.filter)
		};

		tracing_subscriber::fmt::layer()
			.with_ansi(false)
			.with_writer(file_appender)
			.with_file(true)
			.with_line_number(true)
			.with_thread_ids(file.common.verbose)
			.with_target(true)
			.with_filter(filter)
	});

	let remote = &config.remote;
	let remote = remote
		.enable
		.then(|| {
			let mut exporter = opentelemetry_otlp::new_exporter().tonic();
			if !remote.collector_endpoint.is_empty() {
				exporter = exporter.with_endpoint(&remote.collector_endpoint);
			}

			let tracer = opentelemetry_otlp::new_pipeline()
				.tracing()
				.with_exporter(exporter)
				.install_batch(opentelemetry_sdk::runtime::Tokio)?
				.tracer(config.app_name.clone());

			TraceResult::Ok(tracing_opentelemetry::layer().with_tracer(tracer))
		})
		.transpose()?;

	Registry::default().with(console).with(file).with(remote).try_init()?;
	Ok(())
}
