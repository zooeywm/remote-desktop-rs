use crate::{container::Container, domain::{Bar, Foo}, service::{BarImpl, FooImpl}};

impl Foo for Container {
	fn foo(&self) { FooImpl::inj_ref(self).foo() }
}

impl Bar for Container {
	fn bar(&self) { BarImpl::inj_ref(self).bar() }
}
