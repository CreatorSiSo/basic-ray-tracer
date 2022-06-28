use glam::Vec3A;

use crate::ray::Ray;

#[derive(Debug, Default)]
pub struct HitRecord {
	pub point: Vec3A,
	pub normal: Vec3A,
	pub dist: f32,
}

impl HitRecord {
	pub fn new(point: Vec3A, normal: Vec3A, dist: f32) -> Self {
		Self {
			point,
			normal,
			dist,
		}
	}
}

pub trait Hittable {
	fn hit(&self, ray: Ray) -> Option<HitRecord>;
}
