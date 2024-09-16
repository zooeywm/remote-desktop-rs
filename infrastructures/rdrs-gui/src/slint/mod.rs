slint::include_modules!();

use i_slint_backend_winit::{winit::event::WindowEvent, WinitWindowAccessor, WinitWindowEventResult};
use rdrs_domain_player::{service::{Renderer, RendererGenerator}, vo::VideoInfo};
use rdrs_tools::error::Result;
use slint::{Weak, Window};
use tokio::time::Instant;
use tracing::{debug, trace};

#[derive(dep_inj::DepInj)]
#[target(SlintGui)]
pub struct SlintGuiState {
	app: App,
}

impl SlintGuiState {
	pub fn new() -> Result<Self> {
		slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new()?))?;
		let app = App::new()?;
		let window = app.window();
		window.on_winit_window_event(on_winit_event);
		Ok(Self { app })
	}

	pub fn run(&self) -> Result<()> { Ok(self.app.run()?) }
}

/// Owns a app weak pointer
#[derive(Clone)]
pub struct SlintRenderer {
	renderer_info: VideoInfo,
	app_weak:      Weak<App>,
}

impl Renderer for SlintRenderer {
	fn render(&self, decode_buffer: &[u8]) -> Result<()> {
		let instant = Instant::now();
		let pixel_buffer = self.video_frame_to_pixel_buffer(decode_buffer);
		self.app_weak.upgrade_in_event_loop(move |app| {
			app.set_video_frame(slint::Image::from_rgb8(pixel_buffer));
		})?;
		let elapsed = instant.elapsed();
		debug!("frame: {}ms({}ns)", elapsed.as_millis(), elapsed.as_nanos());
		Ok(())
	}
}

impl<Deps> RendererGenerator for SlintGui<Deps>
where
	Deps: AsRef<SlintGuiState>,
{
	type Target = SlintRenderer;

	fn generate_renderer(&self, renderer_info: VideoInfo) -> Result<Self::Target> {
		Ok(SlintRenderer { renderer_info, app_weak: self.app.as_weak() })
	}
}

/// Handle winit event
fn on_winit_event(_window: &Window, event: &WindowEvent) -> WinitWindowEventResult {
	trace!("winit: {event:?}");
	WinitWindowEventResult::Propagate
}

impl SlintRenderer {
	/// Convert video frame to slint pixel buffer
	fn video_frame_to_pixel_buffer(&self, data: &[u8]) -> slint::SharedPixelBuffer<slint::Rgb8Pixel> {
		let info @ VideoInfo { width, height, .. } = self.renderer_info;
		let stride = info.stride();
		let mut pixel_buffer = slint::SharedPixelBuffer::<slint::Rgb8Pixel>::new(width, height);

		let decoded_line_iter = data.chunks_exact(stride);
		let slint_pixel_line_iter = pixel_buffer
			.make_mut_bytes()
			.chunks_mut(width as usize * core::mem::size_of::<slint::Rgb8Pixel>());

		for (source_line, dest_line) in decoded_line_iter.zip(slint_pixel_line_iter) {
			dest_line.copy_from_slice(&source_line[..dest_line.len()])
		}

		pixel_buffer
	}
}
