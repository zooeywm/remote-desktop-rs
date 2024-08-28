use crate::domain::Foo;

#[derive(dep_inj::DepInj)]
#[target(FooImpl)]
pub struct FooState {
    bar: u64,
}

impl FooState {
    pub fn new(bar: u64) -> Self {
        Self { bar }
    }
}

impl<Deps> Foo for FooImpl<Deps>
where
    Deps: AsRef<FooState>,
{
    fn foo(&self) {
        println!("{}", self.bar);
    }
}
