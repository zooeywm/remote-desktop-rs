use std::path::PathBuf;

use rdrs_domain_player::vo::{PixelFormat, StreamSource, VideoInfo};
use rdrs_slint::init;
use rdrs_tools::error::Result;

#[tokio::main(worker_threads = 32)]
async fn main() -> Result<()> {
	let mut container = init()?;

	let video_path = container.extends.clone().into_table()?.get("video_path").unwrap().to_string();

	container.start_decoding_streaming(
		StreamSource::File { path: PathBuf::from(video_path) },
		VideoInfo::new(2880, 1800, PixelFormat::Rgb24),
	)?;

	// std::thread::sleep(Duration::from_millis(1000));
	// container.stop_codec_by_id(0)?;
	// container.stop_all_codecs()?;

	container.start_gui()?;
	Ok(())
}
