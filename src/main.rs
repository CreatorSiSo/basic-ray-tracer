#![deny(clippy::all)]

use glam::{vec3a, vec4};
use hittable::{HitRecord, Hittable};
use std::fs::File;
use std::io::BufWriter;

mod hittable;
mod primitive;
mod ray;
mod renderer;
mod scene;
use primitive::{Primitive, Sphere};
use ray::Ray;
use renderer::{CpuRenderer, Renderer};
use scene::Scene;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

fn main() -> Result<(), std::io::Error> {
	let mut renderer = CpuRenderer::new(WIDTH as f32, HEIGHT as f32);
	let mut scene = Scene::default();
	scene.push(Primitive::Sphere(Sphere::new(vec3a(0., 0., -1.), 0.5)));
	scene.push(Primitive::Sphere(Sphere::new(vec3a(0., -100., -50.), 100.)));

	renderer.render_layer(|_, coord| {
		let ray = Ray {
			origin: vec3a(0., 0., 0.),
			dir: vec3a(coord.x, coord.y, -1.).normalize(),
		};

		if let Some(HitRecord { normal, .. }) = scene.hit(&ray) {
			let normal = (normal + 1.) * 0.5;
			return vec4(normal.x, normal.y, normal.z, 1.);
		}

		vec4(coord.x, coord.y, 0., 0.4)
		// vec4(0., 0., 0., 1.0)
	});

	// Write final image to png file
	let path = "/home/creatorsiso/dev/basic-ray-tracer/result.png";
	let file = File::create(path)?;
	let w = BufWriter::new(file);

	let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
	encoder.set_color(png::ColorType::Rgba);
	encoder.set_depth(png::BitDepth::Eight);

	let mut writer = encoder.write_header()?;

	writer.write_image_data(renderer.final_image().as_slice())?;

	Ok(())
}
