use bevy::prelude::*;
use super::SimpleCollider;
use super::AABB;

pub(crate) struct Point {
	pub position: Vec2,
}

impl SimpleCollider for Point {
	fn get_position(&self) -> Vec2 {
	    self.position
	}

	fn closest_point(&self, _: Vec2) -> Vec2 {
	   self.position
	}

	fn into_aabb(&self, _: Vec2) -> AABB {
		AABB {
			position: self.position,
			min: self.position,
			max: self.position,
		}
	}

	fn collide(&self, other: Box<dyn SimpleCollider>) -> bool {
	    let AABB { min, max, .. } = other.into_aabb(other.get_position());
		let adif = min - self.position;
		let bdif = self.position - max;
		if adif.x > 0. || adif.y > 0. { return false }
		if bdif.x > 0. || bdif.y > 0. { return false }
		true
	}
}

