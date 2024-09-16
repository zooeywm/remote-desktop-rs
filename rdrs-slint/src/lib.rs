use rdrs_kernel::{config::{build_config, CommonConfig}, Container};
use rdrs_tools::error::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SlintConfig {
	#[serde(default, flatten)]
	pub common: CommonConfig,
}

pub fn init() -> Result<Container> {
	let SlintConfig { common } = build_config()?.try_deserialize()?;
	let container = Container::initialize(common)?;
	Ok(container)
}
