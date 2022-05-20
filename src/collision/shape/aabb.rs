use bevy::prelude::*;
use super::SimpleCollider;

#[derive(Clone)]
pub(crate) struct AABB {
	pub position: Vec2,
	pub min: Vec2,
	pub max: Vec2,
}

impl SimpleCollider for AABB {
	fn get_position(&self) -> Vec2 {
		self.position
	}

	fn closest_point(&self, point: Vec2) -> Vec2 {
		let AABB { min, max, .. } = *self;
		min.max(max.min(point))
	}

	fn into_aabb(&self, _: Vec2) -> AABB {
		self.clone()
	}
}

