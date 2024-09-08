use std::path::PathBuf;

use rdrs_kernel::{model::StreamType, Result};
use rdrs_slint::init;

#[tokio::main(worker_threads = 32)]
async fn main() -> Result<()> {
	let container = init()?;

	let video_path =
		container.extends.clone().unwrap().into_table()?.get("video_path").unwrap().to_string();

	container.start_decoding_streaming(StreamType::File { path: PathBuf::from(video_path) })?;

	// std::thread::sleep(Duration::from_millis(1000));
	// container.stop_codec_by_id(0)?;
	// container.stop_all_codecs()?;

	container.start_gui()?;
	Ok(())
}
