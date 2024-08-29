mod boilerplate;

use remote_desk_config::AppConfig;

use crate::infras::FooServiceState;

#[derive(derive_more::AsRef)]
pub struct Container {
	#[as_ref]
	foo_service: FooServiceState,
}

impl Container {
	pub fn new(config: &AppConfig) -> Self {
		let foo_service = FooServiceState::new(config.num);
		Self { foo_service }
	}
}
