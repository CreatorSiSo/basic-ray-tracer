use glam::Vec4;

use crate::camera::Camera;
use crate::utils::default;

pub trait Renderer {
	fn resize_surface(&mut self, width: u32, height: u32);
	// fn update(&mut self);
	fn render(&mut self);
	fn final_image(&self) -> Vec<u8>;
}

pub struct CpuRenderer {
	pub camera: Camera,
	pub surface: Vec<Vec4>,
	finished: bool,
}

impl CpuRenderer {
	pub fn new(camera: Camera) -> Self {
		match camera {
			Camera::Orthographic { width, height } | Camera::_Perspective { width, height } => Self {
				camera,
				surface: vec![Vec4::ZERO; width as usize * height as usize],
				finished: false,
			},
		}
	}
}

impl Renderer for CpuRenderer {
	fn resize_surface(&mut self, width: u32, height: u32) {
		self.finished = false;
		self
			.surface
			.resize_with((width * height) as usize, || default());
		self.camera.resize(width, height);
	}

	fn render(&mut self) {
		self.finished = true;
	}

	fn final_image(&self) -> Vec<u8> {
		self
			.surface
			.iter()
			.flat_map(|color| {
				[
					(color.x * 255.0) as u8,
					(color.y * 255.0) as u8,
					(color.z * 255.0) as u8,
					(color.w * 255.0) as u8,
				]
				.into_iter()
			})
			.collect()
	}
}
