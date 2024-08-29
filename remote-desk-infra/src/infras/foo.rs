use remote_desk_core::service::FooService;

#[derive(dep_inj::DepInj)]
#[target(FooServiceImpl)]
pub struct FooServiceState {
	bar: u64,
}

impl FooServiceState {
	pub fn new(bar: u64) -> Self { Self { bar } }
}

impl<Deps> FooService for FooServiceImpl<Deps>
where
	Deps: AsRef<FooServiceState>,
{
	fn foo(&self) {
		println!("{}", self.bar);
	}
}
