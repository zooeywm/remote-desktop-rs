use crate::error::Result;

/// Gui service
pub trait Gui {
	/// Block to run the app (usually on main thread).
	fn run(&self) -> Result<()>;
}
