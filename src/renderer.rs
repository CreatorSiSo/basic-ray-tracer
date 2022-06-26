pub struct Vec2 {
	pub u: f32,
	pub v: f32,
}

// pub struct Vec3 {
// 	pub x: f32,
// 	pub y: f32,
// 	pub z: f32,
// }

#[derive(Debug, Default, Clone)]
pub struct Vec4 {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

pub trait Renderer {
	// fn resize_surface(&mut self, width: u32, height: u32);
	// fn update(&mut self);

	fn render(&mut self, shader: impl Fn(&Vec4, Vec2) -> Vec4);

	fn final_image(&self) -> Vec<u8>;
}

pub struct CpuRenderer {
	surface: Vec<Vec4>,
	width: f32,
	height: f32,
}

impl CpuRenderer {
	pub fn new(width: u32, height: u32) -> Self {
		let surface = vec![Vec4::default(); width as usize * height as usize];

		Self {
			surface,
			width: width as f32,
			height: height as f32,
		}
	}
}

impl Renderer for CpuRenderer {
	fn render(&mut self, shader: impl Fn(&Vec4, Vec2) -> Vec4) {
		for index in 0..self.surface.len() {
			self.surface[index] = {
				let coord = Vec2 {
					u: (index as f32 / self.width) % 1.0,
					v: (index as f32 / self.width).floor() / self.height,
				};
				shader(&self.surface[index], coord)
			}
		}
	}

	fn final_image(&self) -> Vec<u8> {
		self
			.surface
			.iter()
			.map(|vec4| {
				[
					(vec4.r * 255.0) as u8,
					(vec4.g * 255.0) as u8,
					(vec4.b * 255.0) as u8,
					(vec4.a * 255.0) as u8,
				]
				.into_iter()
			})
			.flatten()
			.collect()
	}
}
