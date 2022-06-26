#![deny(clippy::all)]

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

mod renderer;
use renderer::Renderer;

use crate::renderer::{CpuRenderer, Vec4};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn main() -> Result<(), Box<dyn Error>> {
	let path = env::args()
		.nth(1)
		.expect("Expected a filename to output to.");

	let mut renderer = CpuRenderer::new(WIDTH, HEIGHT);
	renderer.render(|_, coord| Vec4 {
		r: coord.u,
		g: coord.v * 0.5,
		b: 0.0,
		a: 1.0,
	});

	renderer.render(|previous, _| {
		let mut result = previous.clone();
		result.r *= 0.75;
		result.b = 0.6;
		result
	});

	let file = File::create(path).unwrap();
	let ref mut w = BufWriter::new(file);

	let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT); // Width is 2 pixels and height is 1.
	encoder.set_color(png::ColorType::Rgba);
	encoder.set_depth(png::BitDepth::Eight);

	let mut writer = encoder.write_header().unwrap();

	let data = renderer.final_image();
	writer.write_image_data(data.as_slice()).unwrap();

	Ok(())
}
