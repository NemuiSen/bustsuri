use bevy::prelude::*;
use super::SimpleCollider;
use super::AABB;

pub(crate) struct Circle {
	pub position: Vec2,
	pub radius: f32,
}

impl SimpleCollider for Circle {
	fn get_position(&self) -> Vec2 {
	    self.position
	}

	fn closest_point(&self, point: Vec2) -> Vec2 {
		let Circle { radius, position } = *self;
		let dif = point - position;
		let mag = dif.length();
		position+radius.min(mag)*dif/mag
	}

	fn into_aabb(&self, point: Vec2) -> AABB {
		let size = (self.closest_point(point) - self.position).abs();
		AABB {
			position: self.position,
			min: self.position-size,
			max: self.position+size,
		}
	}
}

