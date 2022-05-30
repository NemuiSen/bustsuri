use bevy::prelude::*;
use crate::rigid_body::{IsSleep, IsStatic};

// v=d*t
/// Required components for work: [`Transform`]
#[derive(Default, Component)]
pub struct Velocity {
	/// Affects the position
	pub linear: Vec2,
	/// Affects the rotation
	pub angular: f32,
}

// a=v*t
/// If you need to handle the Acceleration manually and you dont need the component [`Force`]
/// just remove the bundle [`ForcesBundle`], but if you still require the component you will have
/// to transform your acceleration into a force, just multiply the acceleration by the mass of the entity
/// F = m * a
/// Required components for work: [`Velocity`]
#[derive(Default, Component)]
pub struct Acceleration {
	/// Affects the position
	pub linear: Vec2,
	/// Affects the rotation
	pub angular: f32,
}

// F=m*a
/// Required components for work: [`Acceleration`]
#[derive(Default, Component)]
pub struct Force {
	/// Affects the position
	pub linear: Vec2,
	/// Affects the rotation
	pub angular: f32,
}

/// Object resistance
/// Required components for work: [`Force`]
#[derive(Component)]
pub struct Resistance {
	pub mass: f32,
	pub inertia: f32,
}

impl Default for Resistance {
	fn default() -> Self {
	    Self {
			mass: 1.,
			inertia: 1.,
		}
	}
}

// |u|^2 * c * û * -1
/// Enviroment resistance
/// Required components for work: [`Force`], [`Velocity`]
#[derive(Default, Component)]
pub struct Drag {
	pub linear: f32,
	pub angular: f32,
}

#[derive(Default, Bundle)]
pub struct DynamicBundle {
	pub velocity: Velocity,
	pub acceleration: Acceleration,
}

#[derive(Default, Bundle)]
pub struct ForcesBundle {
	pub force: Force,
	pub resistance: Resistance,
	pub drag: Drag,
}

#[derive(Default, Bundle)]
pub struct KinematicBundle {
	#[bundle] pub dynamic: DynamicBundle,
	#[bundle] pub forces: ForcesBundle,
}

pub(crate) fn update_transform (
	time: Res<Time>,
	mut query: Query<(
		&mut Transform,
		// Dynamics
		&mut Velocity,
		&mut Acceleration,
		// Forces
		Option<(&mut Force, &Resistance, &Drag)>,
		// RigidBody
		Option<&IsStatic>,
		Option<&IsSleep>,
	)>
) {
	for (
		mut transform,
		mut velocity,
		mut acceleration,
		forces,
		is_static,
		is_sleep,
	) in query.iter_mut() {
		// Stop when is static
		if let Some(is_static) = is_static { if **is_static { return; } }
		let delta = time.delta_seconds();
		if let Some((mut force, resistance, drag)) = forces {
			// Drag -> Force
			// Linear
			let linear_speed_speed = velocity.linear.length_squared();
			let linear_drag_magnitude = drag.linear * linear_speed_speed;
			let linear_drag = -velocity.linear.normalize_or_zero() * linear_drag_magnitude;
			force.linear  += linear_drag;
			// Angular
			let angular_speed_speed = velocity.angular * velocity.angular;
			let angular_drag_magnitude = drag.angular * angular_speed_speed;
			let angular_drag = -velocity.angular * angular_drag_magnitude;
			force.angular += angular_drag;
			// Force -> Acceleration
			acceleration.linear  = force.linear  / resistance.mass   ;
			acceleration.angular = force.angular / resistance.inertia;
		}
		// Acceleration -> Velocity
		velocity.linear  += acceleration.linear  * delta;
		velocity.angular += acceleration.angular * delta;
		// Stop when the change is near to zero
		if let Some(is_sleep) = is_sleep { if **is_sleep { return; } }
		// Velocity -> Transform
		transform.translation += velocity.linear.extend(0.0) * delta;
		transform.rotate(Quat::from_rotation_z(velocity.angular * delta));
	}
}

pub(crate) fn is_sleep(
	mut query: Query<(&Velocity, &Acceleration, &Force, &mut IsSleep)>
) {
	for (velocity, acceleration, force, mut is_sleep) in query.iter_mut() {
		let sleep =
			tend_zero_vec2(    velocity.linear) || tend_zero_f32(    velocity.angular) ||
			tend_zero_vec2(acceleration.linear) || tend_zero_f32(acceleration.angular) ||
			tend_zero_vec2(       force.linear) || tend_zero_f32(       force.angular);
		if **is_sleep != sleep { **is_sleep = sleep; }
	}
}

fn tend_zero_f32(n: f32) -> bool {
	n.abs() < f32::EPSILON 
}

fn tend_zero_vec2(v: Vec2) -> bool {
	v.x.abs() < f32::EPSILON && v.y.abs() < f32::EPSILON
}

