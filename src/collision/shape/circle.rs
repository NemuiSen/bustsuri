use bevy::prelude::*;
use super::Collider;

pub(crate) struct Circle {
	pub position: Vec2,
	pub radius: f32,
}

impl Collider for Circle {
	fn get_positions(&self) -> Vec<Vec2> {
		vec![self.position]
	}

	fn axes_from_position(&self, other_positions: &Vec<Vec2>) -> Vec<Vec2> {
		other_positions.iter()
			.map(|&p| (p-self.position)
			.normalize_or_zero())
			.collect()
	}

	fn range_along_axis(&self, axis_proj: Vec2) -> (f32, f32) {
		let vmin = self.position - axis_proj*self.radius;
		let vmax = self.position + axis_proj*self.radius;
		(vmin.dot(axis_proj), vmax.dot(axis_proj))
	}
}

