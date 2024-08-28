mod boilerplate;

use crate::{service::FooState, AppConfig};

#[derive(derive_more::AsRef)]
pub struct Container {
    #[as_ref]
    foo: FooState,
}

impl Container {
    pub fn new(config: &AppConfig) -> Self {
        let foo2 = FooState::new(config.num);
        Self { foo: foo2 }
    }
}
