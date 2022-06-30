use glam::vec3a;

use crate::ray::Ray;

/// TODO
pub enum Camera {
	Orthographic,
	_Perspective,
}

impl Camera {
	pub fn get_ray(&self, u: f32, v: f32) -> Ray {
		match self {
			Camera::Orthographic { .. } => Ray {
				origin: vec3a(u, v, 0.),
				dir: vec3a(0., 0., -1.),
			},
			Camera::_Perspective { .. } => Ray {
				origin: vec3a(0., 0., 0.),
				dir: vec3a(u, v, -1.).normalize(),
			},
		}
	}
}
