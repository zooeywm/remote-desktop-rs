fn main() {
	#[cfg(feature = "slint")]
	slint_build::compile("ui/slint/appwindow.slint").unwrap();
}
