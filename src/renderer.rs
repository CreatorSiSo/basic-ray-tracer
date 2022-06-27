use nalgebra_glm::{vec2, vec4, TVec2, TVec4};

pub trait Renderer {
	// fn resize_surface(&mut self, width: u32, height: u32);
	// fn update(&mut self);

	fn render_layer(&mut self, shader: impl Fn(&TVec4<f32>, TVec2<f32>) -> TVec4<f32>);

	fn final_image(&self) -> Vec<u8>;
}

pub struct CpuRenderer {
	surface: Vec<TVec4<f32>>,
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
	fn render_layer(&mut self, shader: impl Fn(&TVec4<f32>, TVec2<f32>) -> TVec4<f32>) {
		for index in 0..self.surface.len() {
			self.surface[index] = {
				let centered_coord = {
					let coord = vec2(
						(index as f32 / self.width) % 1.0,
						(index as f32 / self.width).floor() / self.height,
					);
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
