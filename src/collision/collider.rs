use bevy::prelude::*;
use super::shape::AABB;

pub(crate) trait SimpleCollider {
	fn get_position(&self) -> Vec2;
	fn closest_point(&self, point: Vec2) -> Vec2;
	fn into_aabb(&self, point: Vec2) -> AABB;
	fn collide(&self, other: Box<dyn SimpleCollider>) -> bool {
		let AABB { min, max, .. } = self.into_aabb(other.get_position());
		let point = other.closest_point(self.get_position());
		let adif = min - point;
		let bdif = point - max;
		if adif.x > 0. || adif.y > 0. { return false }
		if bdif.x > 0. || bdif.y > 0. { return false }
		true
	}
}

