use glam::Vec4;

use crate::camera::Camera;

pub trait Renderer {
	// fn update(&mut self);

	fn resize_surface(&mut self, width: u32, height: u32);
	fn render(&mut self);
	fn final_image(&self) -> Vec<u8>;
}

pub struct CpuRenderer {
	pub camera: Camera,
	pub surface: Vec<Vec4>,
}

impl CpuRenderer {
	pub fn new(camera: Camera) -> Self {
		match camera {
			Camera::Orthographic { width, height } | Camera::_Perspective { width, height } => Self {
				camera,
				surface: vec![Vec4::ZERO; width as usize * height as usize],
			},
		}
	}
}

impl Renderer for CpuRenderer {
	fn resize_surface(&mut self, _width: u32, _heightt: u32) {
		todo!()
	}

	fn render(&mut self) {
		todo!()
	}

	fn final_image(&self) -> Vec<u8> {
		todo!()
	}
}
