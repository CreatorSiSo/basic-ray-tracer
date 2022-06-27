use cgmath::{vec2, vec4, Vector2, Vector4};

pub trait Renderer {
	// fn resize_surface(&mut self, width: u32, height: u32);
	// fn update(&mut self);

	fn render_layer(&mut self, shader: impl Fn(&Vector4<f32>, Vector2<f32>) -> Vector4<f32>);

	fn final_image(&self) -> Vec<u8>;
}

pub struct CpuRenderer {
	surface: Vec<Vector4<f32>>,
	width: f32,
	height: f32,
}

impl CpuRenderer {
	pub fn new(width: f32, height: f32) -> Self {
		let surface = vec![vec4(0., 0., 0., 0.); width as usize * height as usize];

		Self {
			surface,
			width,
			height,
		}
	}
}

impl Renderer for CpuRenderer {
	fn render_layer(&mut self, shader: impl Fn(&Vector4<f32>, Vector2<f32>) -> Vector4<f32>) {
		for index in 0..self.surface.len() {
			self.surface[index] = {
				let centered_coord = {
					let coord = Vector2 {
						x: (index as f32 / self.width) % 1.0,
						y: (index as f32 / self.width).floor() / self.height,
					};
					coord * 2.0 - vec2(1.0, 1.0) // Remap 0..1 to -1..1
				};
				shader(&self.surface[index], centered_coord)
			}
		}
	}

	fn final_image(&self) -> Vec<u8> {
		self
			.surface
			.iter()
			.flat_map(|vec4| {
				[
					(vec4.x * 255.0) as u8,
					(vec4.y * 255.0) as u8,
					(vec4.z * 255.0) as u8,
					(vec4.w * 255.0) as u8,
				]
				.into_iter()
			})
			.collect()
	}
}
