use glam::{vec2, vec4, Vec2, Vec4};

use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable};
use crate::scene::Scene;
use crate::utils::default;

pub trait Renderer {
	fn resize_surface(&mut self, width: f32, height: f32);
	// fn update(&mut self);
	fn render(&mut self);
	fn final_image(&self) -> Vec<u8>;
}

pub struct CpuRenderer {
	pub surface: Vec<Vec4>,
	pub camera: Camera,
	pub samples: u16,
	scene: Scene,
	width: f32,
	height: f32,
	finished: bool,
}

impl CpuRenderer {
	pub fn new(width: f32, height: f32, camera: Camera, samples: u16, scene: Scene) -> Self {
		match camera {
			Camera::Orthographic | Camera::_Perspective => Self {
				width,
				height,
				camera,
				samples,
				scene,
				surface: vec![Vec4::ZERO; width as usize * height as usize],
				finished: false,
			},
		}
	}
}

impl Renderer for CpuRenderer {
	fn resize_surface(&mut self, width: f32, height: f32) {
		self.finished = false;
		self.width = width;
		self.height = height;
		self.surface.resize_with((width * height) as usize, default);
	}

	fn render(&mut self) {
		use rand::Rng;
		use rayon::prelude::*;

		let pixels = self.surface.par_iter_mut().enumerate();
		pixels.for_each(|(index, pixel)| {
			let mut rng_iter =
				rand::thread_rng().sample_iter(rand::distributions::Uniform::<f32>::new(0., 1.));

			for _ in 0..self.samples {
				let mut coord = Vec2 {
					// Random offset for antialiasing + Index which increases by one each "column"
					x: ((rng_iter.next().unwrap_or_default() + index as f32) / self.width) % 1.,

					// Random offset for antialiasing + Index which increases by one each "row"
					y: (rng_iter.next().unwrap_or_default() + (index as f32 / self.width).floor())
						/ self.height,
				};

				coord.y = coord.y * -1. + 1.; // Flip y axis
				coord = coord * 2.0 - vec2(1.0, 1.0); // Remap 0..1 to -1..1

				let aspect_ratio = self.width / self.height;
				coord.x *= aspect_ratio;

				let ray = self.camera.get_ray(coord.x, coord.y);

				if let Some(HitRecord { normal, .. }) = self.scene.hit(&ray) {
					*pixel += vec4(normal.x, normal.y, normal.z, 1.);
					continue;
				}

				*pixel += vec4(coord.x, coord.y, 0., 0.5);
				// *pixel += vec4(0., 0., 0., 1.0);
			}
		});

		self.finished = true;
	}

	fn final_image(&self) -> Vec<u8> {
		self
			.surface
			.iter()
			.flat_map(|color| {
				let scaled_color = *color * (1. / self.samples as f32);
				[
					(scaled_color.x * 255.0) as u8,
					(scaled_color.y * 255.0) as u8,
					(scaled_color.z * 255.0) as u8,
					(scaled_color.w * 255.0) as u8,
				]
				.into_iter()
			})
			.collect()
	}
}
