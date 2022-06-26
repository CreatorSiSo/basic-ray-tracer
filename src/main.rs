#![deny(clippy::all)]

use cgmath::vec4;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

mod renderer;
use renderer::{CpuRenderer, Renderer};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn main() -> Result<(), Box<dyn Error>> {
	let path = "/home/creatorsiso/dev/basic_ray_tracer/result.png";

	let mut renderer = CpuRenderer::new(WIDTH as f32, HEIGHT as f32);
	renderer.render(|_, coord| vec4(coord.x, coord.y * 0.5, 0.0, 1.0));

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
