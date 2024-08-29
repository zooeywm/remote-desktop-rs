use remote_desk_core::service::FooService;

#[dep_inj_target::dep_inj_target]
pub struct BarServiceImpl;

impl<Deps> BarServiceImpl<Deps>
where
	Deps: FooService,
{
	pub fn bar(&self) {
		// Or (self.prj_ref() as &dyn Foo).foo()
		self.prj_ref().foo()
	}
}
