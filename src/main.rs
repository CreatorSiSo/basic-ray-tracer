#![deny(clippy::all)]

use glam::{vec3a, vec4};
use hittable::{HitRecord, Hittable};
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

mod hittable;
mod object;
mod ray;
mod renderer;
use object::Sphere;
use ray::Ray;
use renderer::{CpuRenderer, Renderer};

const WIDTH: u32 = 2000;
const HEIGHT: u32 = 2000;

fn main() -> Result<(), Box<dyn Error>> {
	let mut renderer = CpuRenderer::new(WIDTH as f32, HEIGHT as f32);
	let sphere = Sphere::new(vec3a(0., 0., -1.), 0.5);

	renderer.render_layer(|_, coord| {
		let ray = Ray {
			origin: vec3a(0., 0., 0.),
			dir: vec3a(coord.x, coord.y, -1.).normalize(),
		};

		if let Some(HitRecord { normal, .. }) = sphere.hit(ray) {
			let normal = (normal + 1.) * 0.5;
			return vec4(normal.x, normal.y, normal.z, 1.);
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
