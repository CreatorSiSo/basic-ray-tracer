#![deny(clippy::all)]

use glam::{vec2, vec3a, vec4, Vec2, Vec4};
use std::fs::File;
use std::io::BufWriter;

mod camera;
mod hittable;
mod primitive;
mod ray;
mod renderer;
mod scene;
use camera::Camera;
use hittable::{HitRecord, Hittable};
use primitive::{Primitive, Sphere};
use renderer::CpuRenderer;
use scene::Scene;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

fn main() -> Result<(), std::io::Error> {
	let camera = Camera::Orthographic {
		width: WIDTH,
		height: HEIGHT,
	};
	let mut renderer = CpuRenderer::new(camera);

	let mut scene = Scene::default();
	scene.push(Primitive::Sphere(Sphere::new(vec3a(0., 0., -1.), 0.5)));
	scene.push(Primitive::Sphere(Sphere::new(vec3a(0.6, 0.3, -2.), 1.)));

	let shader = |_previous: &Vec4, coord: Vec2| -> Vec4 {
		let ray = renderer.camera.get_ray(coord.x, coord.y);

		if let Some(HitRecord { normal, .. }) = scene.hit(&ray) {
			let normal = (normal + 1.) * 0.5;
			return vec4(normal.x, normal.y, normal.z, 1.);
		}

		vec4(coord.x, coord.y, 0., 0.4)
		// vec4(0., 0., 0., 1.0)
	};

	for (index, pixel) in renderer.surface.iter_mut().enumerate() {
		let centered_coord = {
			let mut coord = vec2(
				(index as f32 / WIDTH as f32) % 1.,
				(index as f32 / WIDTH as f32).floor() / HEIGHT as f32,
			);
			coord.y = coord.y * -1. + 1.; // Flip y axis
			coord * 2.0 - vec2(1.0, 1.0) // Remap 0..1 to -1..1
		};
		*pixel += shader(pixel, centered_coord);
	}

	let final_image: Vec<u8> = renderer
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
		.collect();

	write_file(
		"/home/creatorsiso/dev/basic-ray-tracer/result.png",
		final_image.as_slice(),
	)
}

fn write_file<P>(path: P, data: &[u8]) -> Result<(), std::io::Error>
where
	P: std::convert::AsRef<std::path::Path>,
{
	let file = File::create(path)?;
	let w = BufWriter::new(file);

	let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);
	encoder.set_color(png::ColorType::Rgba);
	encoder.set_depth(png::BitDepth::Eight);

	let mut writer = encoder.write_header()?;
	writer.write_image_data(data)?;

	Ok(())
}
