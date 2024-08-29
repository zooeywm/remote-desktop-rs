pub mod container;
mod infras;

use container::Container;
use remote_desk_config::AppConfig;
use remote_desk_core::service::BarService;

fn init_container() -> Container { Container::new(&AppConfig::new(1)) }

pub fn call_foo_from_bar() { init_container().bar() }
