mod boilerplate;

use remote_desk_config::AppConfig;

#[derive(derive_more::AsRef)]
pub struct Container {}

impl Container {
	pub fn new(_config: &AppConfig) -> Self { Self {} }
}
