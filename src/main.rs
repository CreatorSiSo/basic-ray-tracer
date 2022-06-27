#![deny(clippy::all)]

use nalgebra_glm::{vec3, vec4, TVec3};
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

mod renderer;
mod types;
use renderer::{CpuRenderer, Renderer};
use types::Ray;

const WIDTH: u32 = 2000;
const HEIGHT: u32 = 2000;

// TODO: Use tuple of (Option<f32>, Option<f32>)
fn hit_sphere(center: TVec3<f32>, radius: f32, ray: &Ray<f32>) -> Option<f32> {
	let moved_origin = ray.origin - center;

	let a = nalgebra_glm::dot(&ray.dir, &ray.dir);
	let b = 2. * nalgebra_glm::dot(&moved_origin, &ray.dir);
	let c = nalgebra_glm::dot(&moved_origin, &moved_origin) - radius * radius;

	let discriminant = b * b - 4. * a * c;

	let first = (-1. * b - (discriminant).sqrt()) / 2. * a;
	let _second = (-1. * b + (discriminant).sqrt()) / 2. * a;

	return if discriminant >= 0. {
		Some(first)
	} else {
		None
	};
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut renderer = CpuRenderer::new(WIDTH as f32, HEIGHT as f32);
	renderer.render_layer(|_, coord| {
		let ray = Ray {
			origin: vec3(0., 0., 0.),
			dir: vec3(coord.x, coord.y, -1.).normalize(),
		};

		if let Some(dist) = hit_sphere(vec3(0., 0., -1.), 0.5, &ray) {
			let normal = ray.at(dist) - vec3(0., 0., -1.);
			return vec4(normal.x + 1., normal.y + 1., normal.z + 1., 2.) * 0.5;
		}

		vec4(coord.x, coord.y, 0., 0.4)
		// vec4(0., 0., 0., 1.0)
	});

	// Write final image to png file
	let path = "/home/creatorsiso/dev/basic_ray_tracer/result.png";
	let file = File::create(path)?;
	let w = BufWriter::new(file);

	let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
	encoder.set_color(png::ColorType::Rgba);
	encoder.set_depth(png::BitDepth::Eight);

	let mut writer = encoder.write_header()?;

	writer.write_image_data(renderer.final_image().as_slice())?;

	Ok(())
}
