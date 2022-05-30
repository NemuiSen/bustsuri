mod collider;
pub mod shape {
	use super::collider::*;
	//mod aabb; pub(super) use aabb::AABB;
	mod circle; pub(super) use circle::Circle;
	//mod point; pub(super) use point::Point;
	mod polygon; pub(super) use polygon::Polygon;
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
	Square(f32, f32),
	Circle(f32),
	Polygon(Vec<Vec2>),
}

impl ColliderShape {
	fn as_collider(&self, trans: GlobalTransform) -> Box<dyn collider::Collider> {
		match self {
			&Self::Circle(radius) => Box::new(shape::Circle { radius, position: trans.translation.truncate() }),
			&Self::Square(w, h) => Box::new(shape::Polygon::square(w, h, trans)),
			Self::Polygon(v) => Box::new(shape::Polygon::from_vertices(v.clone(), trans)),
			_ => unimplemented!()
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
		let acoll = ashape.as_collider(*atrans);
		let bcoll = bshape.as_collider(*btrans);
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

