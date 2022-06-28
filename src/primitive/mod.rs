mod sphere;

pub use sphere::Sphere;

use crate::hittable::{HitRecord, Hittable};

pub enum Primitive {
	Sphere(Sphere),
}

impl Hittable for Primitive {
	fn hit(&self, ray: &crate::ray::Ray) -> Option<HitRecord> {
		match self {
			Primitive::Sphere(sphere) => sphere.hit(ray),
		}
	}
}
