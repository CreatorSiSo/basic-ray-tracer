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

const WIDTH: u32 = 3840;
const WIDTH_F: f32 = WIDTH as f32;
const HEIGHT: u32 = 2160;
const HEIGHT_F: f32 = HEIGHT as f32;

fn main() -> Result<(), std::io::Error> {
	let mut scene = Scene::default();
	scene.push(Primitive::Sphere(Sphere::new(vec3a(0., 0., -1.), 0.5)));
	scene.push(Primitive::Sphere(Sphere::new(vec3a(0.6, 0.3, -2.), 1.)));

	let mut renderer = CpuRenderer::new(WIDTH_F, HEIGHT_F, Camera::Orthographic, 128, scene);

	renderer.render();

	write_file("./result.png", renderer.final_image().as_slice())
}

fn write_file<P>(path: P, data: &[u8]) -> Result<(), std::io::Error>
where
	P: std::convert::AsRef<std::path::Path>,
{
	let file = File::create(path)?;
	let file_writer = BufWriter::new(file);

	let mut encoder = png::Encoder::new(file_writer, WIDTH, HEIGHT);
	encoder.set_color(png::ColorType::Rgba);
	encoder.set_depth(png::BitDepth::Eight);

	let mut enc_writer = encoder.write_header()?;
	enc_writer.write_image_data(data)?;

	Ok(())
}
