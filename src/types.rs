use nalgebra_glm::{Scalar, TVec3};

pub struct Ray<S> {
	pub origin: TVec3<S>,
	pub dir: TVec3<S>,
}

impl<S> Ray<S>
where
	S: Scalar + nalgebra::ComplexField<RealField = S>,
{
	pub fn at(&self, t: S) -> TVec3<S> {
		&self.origin + self.dir.scale(t)
	}
}

#[cfg(test)]
mod test {
	use super::Ray;
	use nalgebra_glm::vec3;

	#[test]
	fn ray_creation() {
		let ray: Ray<f32> = Ray {
			origin: vec3(0., 0., 0.),
			dir: vec3(0., 1., 0.),
		};

		assert_eq!(ray.at(2.19023), vec3(0., 2.19023, 0.));
		assert_eq!(ray.at(0.78345), vec3(0., 0.78345, 0.));
	}
}
