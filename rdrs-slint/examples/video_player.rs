use std::path::PathBuf;

use rdrs_kernel::{model::StreamSource, Result};
use rdrs_slint::init;

#[tokio::main(worker_threads = 32)]
async fn main() -> Result<()> {
	let mut container = init()?;

	let video_path =
		container.extends.take().unwrap().into_table()?.get("video_path").unwrap().to_string();

	container.start_decode(StreamSource::File { path: PathBuf::from(video_path) })?;

	container.start_gui()
}
