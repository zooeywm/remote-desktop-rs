mod boilerplate;

use crate::{service::FooState, AppConfig};

#[derive(derive_more::AsRef)]
pub struct Container {
	#[as_ref]
	foo: FooState,
}

impl Container {
	pub fn new(config: &AppConfig) -> Self { Self { foo: FooState::new(config.num) } }
}
