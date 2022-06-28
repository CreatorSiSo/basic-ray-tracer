use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::primitive::Primitive;
use crate::ray::Ray;

#[derive(Default)]
pub struct Scene {
	pub primitives: Vec<Arc<Primitive>>,
}

impl Scene {
	pub fn push(&mut self, primitive: Primitive) {
		self.primitives.push(Arc::new(primitive))
	}

	pub fn _clear(&mut self) {
		self.primitives.clear()
	}
}

impl Hittable for Scene {
	fn hit(&self, ray: &Ray) -> Option<HitRecord> {
		for arc in &self.primitives {
			let primitive = arc.as_ref();
			let hit_record = primitive.hit(ray);
			if hit_record.is_some() {
				return hit_record;
			}
		}

		None
	}
}
