use crate::domain::Foo;

#[dep_inj_target::dep_inj_target]
pub struct BarImpl;

impl<Deps> BarImpl<Deps>
where
    Deps: Foo,
{
    pub fn bar(&self) {
        // Or (self.prj_ref() as &dyn Foo).foo()
        self.prj_ref().foo()
    }
}
