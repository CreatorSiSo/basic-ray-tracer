#![deny(clippy::all)]

use glam::vec3a;
use std::fs::File;
use std::io::BufWriter;

mod camera;
mod hittable;
mod primitive;
mod ray;
mod renderer;
mod scene;
mod utils;
use camera::Camera;
use primitive::{Primitive, Sphere};
use renderer::{CpuRenderer, Renderer};
use scene::Scene;

const WIDTH: u32 = 128;
const WIDTH_F: f32 = WIDTH as f32;
const HEIGHT: u32 = 128;
const HEIGHT_F: f32 = HEIGHT as f32;

fn main() -> Result<(), std::io::Error> {
	let mut scene = Scene::default();
	scene.push(Primitive::Sphere(Sphere::new(vec3a(0., 0., -1.), 0.5)));
	scene.push(Primitive::Sphere(Sphere::new(vec3a(0.6, 0.3, -2.), 1.)));

	let mut renderer = CpuRenderer::new(WIDTH_F, HEIGHT_F, Camera::Orthographic, 128, scene);

	renderer.render();

	write_file(
		"/home/creatorsiso/dev/basic-ray-tracer/result.png",
		renderer.final_image().as_slice(),
	)
}

fn write_file<P>(path: P, data: &[u8]) -> Result<(), std::io::Error>
where
	P: std::convert::AsRef<std::path::Path>,
{
	let file = File::create(path)?;
	let w = BufWriter::new(file);

	let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
	encoder.set_color(png::ColorType::Rgba);
	encoder.set_depth(png::BitDepth::Eight);

	let mut writer = encoder.write_header()?;
	writer.write_image_data(data)?;

	Ok(())
}
