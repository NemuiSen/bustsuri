use bevy::prelude::*;
use super::SimpleCollider;
use super::AABB;

pub(crate) struct Point {
	pub position: Vec2,
}

impl Collider for Point {
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
			max: self.position
		}
	}

	fn collide(&self, other: Box<dyn SimpleCollider>) -> bool {
		let other_position = other.closest_point(self.position);

		aprox_vec2_eq(self.position, other_position)
	}
}

fn aprox_vec2_eq(p1: Vec2, p2: Vec2) -> bool {
	let d = (p1-p2).abs();
	d.x < f32::EPSILON && d.y < f32::EPSILON
}

