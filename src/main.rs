#![deny(clippy::all)]

use glam::{vec2, vec3a, vec4, Vec2, Vec4};
use rand::Rng;
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
use hittable::{HitRecord, Hittable};
use primitive::{Primitive, Sphere};
use renderer::{CpuRenderer, Renderer};
use scene::Scene;

const WIDTH: u32 = 128;
const WIDTH_F: f32 = WIDTH as f32;
const HEIGHT: u32 = 128;
const HEIGHT_F: f32 = HEIGHT as f32;

fn main() -> Result<(), std::io::Error> {
	let camera = Camera::Orthographic {
		width: WIDTH,
		height: HEIGHT,
	};
	let mut renderer = CpuRenderer::new(camera, 128);

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

	let rng = rand::thread_rng();
	let mut rng_iter = rng.sample_iter(rand::distributions::Uniform::<f32>::new(0., 1.));

	for (index, pixel) in renderer.surface.iter_mut().enumerate() {
		for _ in 0..renderer.samples {
			let mut coord = vec2(
				((
					// Random offset for antialiasing + Index which increases by one each "column"
					rng_iter.next().unwrap_or_default() + index as f32
				) / WIDTH_F)
					% 1.,
				(
					// Random offset for antialiasing + Index which increases by one each "row"
					rng_iter.next().unwrap_or_default() + (index as f32 / WIDTH_F).floor()
				) / HEIGHT_F,
			);
			coord.y = coord.y * -1. + 1.; // Flip y axis
			coord = coord * 2.0 - vec2(1.0, 1.0); // Remap 0..1 to -1..1
			*pixel += shader(pixel, coord);
		}
	}

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
