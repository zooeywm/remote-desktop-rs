slint::include_modules!();

use i_slint_backend_winit::{winit::event::WindowEvent, WinitWindowAccessor, WinitWindowEventResult};
use rdrs_core::{error::Result, model::VideoFrame, service::{Gui, VideoFrameHandler, VideoFrameHandlerGenerator}};
use slint::{Weak, Window};
use tokio::time::Instant;
use tracing::{debug, trace};

#[derive(dep_inj::DepInj)]
#[target(SlintGui)]
pub struct SlintGuiState {
	app: App,
}

pub struct SlintGuiWeak {
	app_weak: Weak<App>,
}

impl SlintGuiState {
	pub fn new() -> Result<Self> {
		slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new()?))?;
		let app = App::new()?;
		let window = app.window();
		window.on_winit_window_event(on_winit_event);
		Ok(Self { app })
	}
}

impl SlintGuiWeak {
	pub fn new(app_weak: Weak<App>) -> Self { Self { app_weak } }
}

impl<Deps> Gui for SlintGui<Deps>
where
	Deps: AsRef<SlintGuiState>,
{
	fn run(&self) -> Result<()> { Ok(self.app.run()?) }
}

impl<Deps> VideoFrameHandlerGenerator for SlintGui<Deps>
where
	Deps: AsRef<SlintGuiState>,
{
	fn generate_video_frame_handler(&self) -> Box<dyn VideoFrameHandler> {
		Box::new(SlintGuiWeak::new(self.app.as_weak()))
	}
}

impl VideoFrameHandler for SlintGuiWeak {
	fn handle_video_frame(&self, video_frame: &dyn VideoFrame) -> Result<()> {
		let app_weak = self.app_weak.clone();
		let instant = Instant::now();
		let pixel_buffer = video_frame_to_pixel_buffer(video_frame);
		app_weak.upgrade_in_event_loop(move |app| {
			app.set_video_frame(slint::Image::from_rgb8(pixel_buffer));
		})?;
		let elapsed = instant.elapsed();
		debug!("frame: {}ms({}ns)", elapsed.as_millis(), elapsed.as_nanos());
		Ok(())
	}
}

/// Handle winit event
fn on_winit_event(_window: &Window, event: &WindowEvent) -> WinitWindowEventResult {
	trace!("winit: {event:?}");
	WinitWindowEventResult::Propagate
}

/// Convert video frame to slint pixel buffer
fn video_frame_to_pixel_buffer(
	frame: &(impl VideoFrame + ?Sized),
) -> slint::SharedPixelBuffer<slint::Rgb8Pixel> {
	let mut pixel_buffer =
		slint::SharedPixelBuffer::<slint::Rgb8Pixel>::new(frame.width(), frame.height());

	let ffmpeg_line_iter = frame.data(0).chunks_exact(frame.stride(0));
	let slint_pixel_line_iter = pixel_buffer
		.make_mut_bytes()
		.chunks_mut(frame.width() as usize * core::mem::size_of::<slint::Rgb8Pixel>());

	for (source_line, dest_line) in ffmpeg_line_iter.zip(slint_pixel_line_iter) {
		dest_line.copy_from_slice(&source_line[..dest_line.len()])
	}

	pixel_buffer
}
