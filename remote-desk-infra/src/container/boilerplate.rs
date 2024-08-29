use remote_desk_core::service::{BarService, FooService};

use crate::{container::Container, infras::{BarServiceImpl, FooServiceImpl}};

impl FooService for Container {
	fn foo(&self) { FooServiceImpl::inj_ref(self).foo() }
}

impl BarService for Container {
	fn bar(&self) { BarServiceImpl::inj_ref(self).bar() }
}
