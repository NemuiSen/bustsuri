mod collider;
pub mod shape {
	use super::collider::*;
	mod   aabb; pub(super) use   aabb::AABB  ;
	mod circle; pub(super) use circle::Circle;
	mod  point; pub(super) use  point::Point ;
}

use bevy::{prelude::*, utils::HashSet};

#[derive(Default, Bundle)]
pub struct ColliderBundle {
	pub shape: ColliderShape,
	     info: CollisionInfo,
}

impl ColliderBundle {
	pub fn new(shape: ColliderShape) -> Self {
		Self {
			shape,
			info: default(),
		}
	}
}

#[derive(Default, Component)]
pub struct CollisionInfo {
	pub is_colliding: bool,
	pub sources: HashSet<Entity>,
}

#[derive(Default, Component)]
pub enum ColliderShape {
	#[default]
	Point,
	AABB(Vec2),
	Circle(f32),
}

impl ColliderShape {
	fn simple_collider(&self, position: Vec2) -> Box<dyn collider::SimpleCollider> {
		match self {
			&ColliderShape::Point => Box::new(shape::Point { position }),
			&ColliderShape::AABB(size) => Box::new(shape::AABB {
				position,
				min: position-size,
				max: position+size,
			}),
			&ColliderShape::Circle(radius)=> Box::new(shape::Circle {
				position,
				radius,
			}),
		}
	}
}

pub(crate) fn collision_info(
	mut query: Query<(Entity, &GlobalTransform, &ColliderShape, &mut CollisionInfo)>
) {
	let mut combinations = query.iter_combinations_mut();
	while let Some([
		(aid, atrans, ashape, mut ainfo),
		(bid, btrans, bshape, mut binfo)
	]) = combinations.fetch_next() {
		let acoll = ashape.simple_collider(atrans.translation.truncate());
		let bcoll = bshape.simple_collider(btrans.translation.truncate());
		if acoll.collide(bcoll) {
			if !ainfo.is_colliding { ainfo.is_colliding = true; }
			if !binfo.is_colliding { binfo.is_colliding = true; }
			if !ainfo.sources.contains(&bid) { ainfo.sources.insert(bid); }
			if !binfo.sources.contains(&aid) { binfo.sources.insert(aid); }
		} else {
			if ainfo.sources.is_empty() && ainfo.is_colliding { ainfo.is_colliding = false; }
			if binfo.sources.is_empty() && binfo.is_colliding { binfo.is_colliding = false; }
			if ainfo.sources.contains(&bid) { ainfo.sources.remove(&bid); }
			if binfo.sources.contains(&aid) { binfo.sources.remove(&aid); }
		}
	}
}

