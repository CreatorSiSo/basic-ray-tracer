use glam::{vec3a, Vec3A};

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Debug)]
pub struct Sphere {
	center: Vec3A,
	radius: f32,
}

impl Sphere {
	pub fn new(center: Vec3A, radius: f32) -> Self {
		Self { center, radius }
	}

	pub fn _default() -> Self {
		Self {
			center: Vec3A::ZERO,
			radius: 1.,
		}
	}
}

impl Hittable for Sphere {
	fn hit(&self, ray: Ray) -> Option<HitRecord> {
		let moved_origin = ray.origin - self.center;

		let a = Vec3A::dot(ray.dir, ray.dir);
		let b = 2. * Vec3A::dot(moved_origin, ray.dir);
		let c = Vec3A::dot(moved_origin, moved_origin) - self.radius * self.radius;

		let discriminant = b * b - 4. * a * c;
		let dist = (-1. * b - /* + */ discriminant.sqrt()) / (2. * a);
		let point = ray.at(dist);
		let normal = point - vec3a(0., 0., -1.);

		if discriminant >= 0. {
			Some(HitRecord::new(point, normal, dist))
		} else {
			None
		}
	}
}
