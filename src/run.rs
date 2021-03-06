// wengwengweng

use crate::*;
use window::*;
use conf::*;

/// Application Context
pub struct Ctx<'a> {
	pub window: &'a mut window::Window,
	pub gfx: &'a mut gfx::Gfx,
	pub app: &'a mut app::App,
	pub audio: &'a mut audio::Audio,
}

/// The Main Trait
pub trait State: 'static + Sized {

	fn init(_: &mut Ctx) -> Result<Self>;

	fn event(&mut self, _: &mut Ctx, _: &input::Event) -> Result<()> {
		return Ok(());
	}

	fn frame(&mut self, _: &mut Ctx) -> Result<()> {
		return Ok(());
	}

	fn quit(&mut self, _: &mut Ctx) -> Result<()> {
		return Ok(());
	}

}

impl Launcher {
	pub fn run<S: State>(self) -> Result<()> {
		return run_with_conf::<S>(self.conf);
	}
}

/// run with configs, see methods under [Launcher](conf::Launcher)
pub fn launcher() -> Launcher {
	return Launcher::default();
}

/// run simple
pub fn run<S: State>() -> Result<()> {
	return launcher().run::<S>();
}

fn run_with_conf<S: State>(conf: conf::Conf) -> Result<()> {

	let mut window = window::Window::new(&conf)?;
	let mut gfx = gfx::Gfx::new(&window, &conf)?;
	let mut app = app::App::new(&conf);
	let mut audio = audio::Audio::new(&conf)?;

	window.swap()?;

	let mut ctx = Ctx {
		window: &mut window,
		gfx: &mut gfx,
		app: &mut app,
		audio: &mut audio,
	};

	let mut s = S::init(&mut ctx)?;

	window.run(move |mut window, e| {

		let mut ctx = Ctx {
			window: &mut window,
			gfx: &mut gfx,
			app: &mut app,
			audio: &mut audio,
		};

		match e {

			WindowEvent::Resize(w, h) => {
				ctx.gfx.resize(w, h);
			},

			WindowEvent::DPIChange(dpi) => {
				ctx.gfx.set_dpi(dpi);
			},

			WindowEvent::Input(e) => {
				s.event(&mut ctx, &e)?;
			},

			WindowEvent::Frame => {
				ctx.app.tick();
				ctx.gfx.begin_frame();
				s.frame(&mut ctx)?;
				ctx.gfx.end_frame();
			},

			WindowEvent::Quit => {
				s.quit(&mut ctx)?;
			},

		}

		return Ok(());

	})?;

	return Ok(());

}

