use remote_desk_rs::{domain::Bar, AppConfig, Container};

fn main() {
	let container = Container::new(&AppConfig::new(1));
	container.bar()
}
