#![deny(clippy::all)]

use cgmath::{vec3, vec4};
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

mod renderer;
use renderer::{CpuRenderer, Renderer};

const WIDTH: u32 = 2000;
const HEIGHT: u32 = 2000;

fn main() -> Result<(), Box<dyn Error>> {
	let mut renderer = CpuRenderer::new(WIDTH as f32, HEIGHT as f32);
	renderer.render_layer(|_, coord| {
		let ray_origin = vec3(0., 0., -1.);
		let ray_direction = vec3(coord.x, coord.y, -1.);
		let sphere_radius = 0.5;

		let a = cgmath::dot(ray_direction, ray_direction);
		let b = 2. * cgmath::dot(ray_origin, ray_direction);
		let c = cgmath::dot(ray_origin, ray_origin) - sphere_radius * sphere_radius;

		let discriminant = b * b - 4. * a * c;

		return if discriminant >= 0. {
			vec4(1., 0., 1., 1.)
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
