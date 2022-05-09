use bevy::{prelude::*, math::vec2};

pub struct CollisionEvent(
	/// Collision destination
	pub Entity,
	/// Collision source
	pub Entity
);

#[derive(Component)]
pub enum ColliderShape {
	Point,
	AABB(f32, f32),
	Circle(f32),
}

pub(crate) fn collision_event(
	mut event: EventWriter<CollisionEvent>,
	query: Query<(Entity, &GlobalTransform, &ColliderShape)>,
) {
	let mut combinations = query.iter_combinations();
	while let Some([(dst, dst_transform, dst_collider), (src, src_transform, src_collider)]) = combinations.fetch_next() {
		if collide(
			dst_collider,
			dst_transform.translation.truncate(),
			src_collider,
			src_transform.translation.truncate(),
		) {
			event.send_batch([
				CollisionEvent(dst, src),
				CollisionEvent(src, dst),
			].into_iter());
		}
	}
}

fn collide(
	a_shape: &ColliderShape, a_position: Vec2,
	b_shape: &ColliderShape, b_position: Vec2,
) -> bool {
	match (a_shape, b_shape) {
		// AABB vs AABB
		(&ColliderShape::AABB(a_w, a_h), &ColliderShape::AABB(b_w, b_h)) => {
			let a_min = a_position - vec2(a_w, a_h);
			let a_max = a_position + vec2(a_w, a_h);
			let b_min = b_position - vec2(b_w, b_h);
			let b_max = b_position + vec2(b_w, b_h);
			let a_dif = a_min - b_max;
			let b_dif = b_min - a_max;
			if a_dif.x > 0. || a_dif.y > 0. { return false }
			if b_dif.x > 0. || b_dif.y > 0. { return false }
			true
			
			//(src_min.x <= dst_max.x && src_max.x >= dst_min.x) &&
			//(src_min.y <= dst_max.y && src_max.y >= dst_min.y)
		},
		// AABB vs Point
		(&ColliderShape::AABB(a_w, a_h), ColliderShape::Point) => {
			let a_min = a_position - vec2(a_w, a_h);
			let a_max = a_position + vec2(a_w, a_h);
			let a_dif = a_min - b_position;
			let b_dif = b_position - a_max;
			if a_dif.x > 0. || a_dif.y > 0. { return false }
			if b_dif.x > 0. || b_dif.y > 0. { return false }
			true
		}
		// Point vs AABB
		(ColliderShape::Point, &ColliderShape::AABB(b_w, b_h)) => {
			let b_min = b_position - vec2(b_w, b_h);
			let b_max = b_position + vec2(b_w, b_h);
			let a_dif =  a_position - b_max;
			let b_dif =  b_min - a_position;
			if a_dif.x > 0. || a_dif.y > 0. { return false }
			if b_dif.x > 0. || b_dif.y > 0. { return false }
			true
		}
		// Circle vs Circle
		(&ColliderShape::Circle(a_radius), &ColliderShape::Circle(b_radius)) => {
			let distance = (a_position - b_position).length();
			distance < a_radius+b_radius
		}
		// Circle vs Point
		(&ColliderShape::Circle(a_radius), &ColliderShape::Point) => {
			let distance = (a_position - b_position).length();
			distance < a_radius
		}
		// Point vs Circle
		(&ColliderShape::Point, &ColliderShape::Circle(b_radius)) => {
			let distance = (b_position - a_position).length();
			distance < b_radius
		}
		// AABB vs Circle
		(&ColliderShape::AABB(a_w, a_h), &ColliderShape::Circle(b_radius)) => {
			let a_min = a_position - vec2(a_w, a_h);
			let a_max = a_position + vec2(a_w, a_h);
			let closest_point = a_min.max(a_max.min(b_position));
			let distance = (closest_point - b_position).length();
			distance < b_radius
		}
		// Circle vs AABB
		(&ColliderShape::Circle(a_radius), &ColliderShape::AABB(b_w, b_h)) => {
			let b_min = b_position - vec2(b_w, b_h);
			let b_max = b_position + vec2(b_w, b_h);
			let closest_point = b_min.max(b_max.min(a_position));
			let distance = (closest_point - a_position).length();
			distance < a_radius
		}
		_ => false
	}
}

//pub(crate) fn collision_movement(
	//mut collision_event: EventReader<CollisionEvent>,
	//mut velocity_query: Query<&mut Velocity>,
	//resistance_query: Query<&Resistance>,
	//body_query: Query<&BodyType>,
//) {
	//for &CollisionEvent(a, b, is_colliding) in collision_event.iter() {
	//	let a_body = body_query.get(a);
	//	let b_body = body_query.get(b);
	//	match (a_body, b_body) {
	//		(Ok(BodyType::Static), _) | (_, Ok(BodyType::Static)) => {
	//			if let Ok(mut a_velocity) = velocity_query.get_mut(a) {
	//				a_velocity.linear = Vec2::ZERO;
	//			}

	//			if let Ok(mut b_velocity) = velocity_query.get_mut(b) {
	//				b_velocity.linear = Vec2::ZERO;
	//			}
	//		},
	//		(Ok(BodyType::Dynamic), Ok(BodyType::Dynamic)) => {
	//			let velocity_a = velocity_query.get(a).unwrap().linear;
	//			let velocity_b = velocity_query.get(b).unwrap().linear;
	//			let Resistance { mass: mass_a, .. } = resistance_query.get(a).unwrap();
	//			let Resistance { mass: mass_b, .. } = resistance_query.get(b).unwrap();
	//			let momentum_a = velocity_a * *mass_a;
	//			let momentum_b = velocity_b * *mass_b;
	//			let momentum_total = momentum_a + momentum_b;
	//			let mass_total = *mass_a + *mass_b;
	//			let velocity_total = momentum_total / mass_total;
	//			velocity_query.get_mut(a).unwrap().linear = velocity_total;
	//			velocity_query.get_mut(b).unwrap().linear = velocity_total;
	//		}
	//		_ => ()
	//	}
	//}
//}

