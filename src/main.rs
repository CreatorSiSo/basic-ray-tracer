#![deny(clippy::all)]

use nalgebra_glm::{vec3, vec3_to_vec4, vec4, TVec4};
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

mod renderer;
mod types;
use renderer::{CpuRenderer, Renderer};
use types::Ray;

const WIDTH: u32 = 2000;
const HEIGHT: u32 = 2000;

fn main() -> Result<(), Box<dyn Error>> {
	let mut renderer = CpuRenderer::new(WIDTH as f32, HEIGHT as f32);
	renderer.render_layer(|_, coord| {
		let ray = Ray {
			origin: vec3(0., 0., -1.),
			dir: vec3(coord.x, coord.y, 1.).normalize(),
		};
		let sphere_radius = 0.7;

		let a = nalgebra_glm::dot(&ray.dir, &ray.dir);
		let b = 2. * nalgebra_glm::dot(&ray.origin, &ray.dir);
		let c = nalgebra_glm::dot(&ray.origin, &ray.origin) - sphere_radius * sphere_radius;

		let discriminant = b * b - 4. * a * c;

		// First hit
		let dist = (-1. * b - (discriminant).sqrt()) / 2. * a;
		// Second hit
		let _dist2 = (-1. * b + (discriminant).sqrt()) / 2. * a;

		let surface_pos = ray.at(dist);

		return if discriminant >= 0. {
			vec4(surface_pos.x, surface_pos.y, surface_pos.z, 1.)
		// vec4(normal.x, normal.y, normal.z, 1.)
		} else {
			vec4(coord.x, coord.y, 0., 0.5)
		};
	});

	// Write final image to png file
	let path = "/home/creatorsiso/dev/basic_ray_tracer/result.png";
	let file = File::create(path)?;
	let w = BufWriter::new(file);

	let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
	encoder.set_color(png::ColorType::Rgba);
	encoder.set_depth(png::BitDepth::Eight);

	let mut writer = encoder.write_header()?;

	let data = renderer.final_image();
	writer.write_image_data(data.as_slice())?;

	Ok(())
}
