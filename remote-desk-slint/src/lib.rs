slint::include_modules!();
use configuration::SlintConfig;
use remote_desk_kernel::{config::build_config, Container, Result};

pub mod configuration;

pub fn init() -> Result<(Container, App)> {
	let SlintConfig { common } = build_config()?.try_deserialize()?;

	let container = Container::initialize(common)?;

	slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new()?))?;

	let app = App::new()?;
	Ok((container, app))
}
