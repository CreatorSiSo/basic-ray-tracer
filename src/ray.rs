use glam::Vec3A;

pub struct Ray {
	pub origin: Vec3A,
	pub dir: Vec3A,
}

impl Ray {
	pub fn at(&self, t: f32) -> Vec3A {
		self.origin + self.dir * t
	}
}

#[cfg(test)]
mod test {
	use super::Ray;
	use glam::vec3a;

	#[test]
	fn ray_creation() {
		let ray: Ray = Ray {
			origin: vec3a(0., 0., 0.),
			dir: vec3a(0., 1., 0.),
		};

		assert_eq!(ray.at(2.19023), vec3a(0., 2.19023, 0.));
		assert_eq!(ray.at(0.78345), vec3a(0., 0.78345, 0.));
	}
}
