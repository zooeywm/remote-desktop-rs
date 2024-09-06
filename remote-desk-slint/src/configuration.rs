use remote_desk_kernel::config::CommonConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SlintConfig {
	#[serde(default, flatten)]
	pub common: CommonConfig,
}
