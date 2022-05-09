use bevy::prelude::*;

/// Move the entity adding its position with the speed, you can update the position of the entity without any problem.
/// Speed can be altered directly or by using acceleration.
#[derive(Default, Component)]
pub struct Velocity {
	/// Affects the position
	pub linear: Vec2,
	/// Affects the rotation
	pub angular: f32,
}

#[derive(Default, Component)]
pub struct Acceleration {
	/// Affects the position
	pub linear: Vec2,
	/// Affects the rotation
	pub angular: f32,
}

#[derive(Default, Component)]
pub struct Force {
	/// Affects the position
	pub linear: Vec2,
	/// Affects the rotation
	pub angular: f32,
}

#[derive(Component)]
pub struct Resistance {
	pub mass: f32,
	pub inertia: f32,
}

impl Default for Resistance {
	fn default() -> Self {
	    Self { mass: 1., inertia: 1. }
	}
}

#[derive(Default, Bundle)]
pub struct Kinematic {
	pub velocity: Velocity,
	pub acceleration: Acceleration,
	pub force: Force,
	pub resistance: Resistance,
}

pub(crate) fn update_transform (
	time: Res<Time>,
	mut query: Query<(
		&mut Transform,
		&mut Velocity,
		&mut Acceleration,
		&Force,
		&Resistance
	)>
) {
	for (
		mut transform,
		mut velocity,
		mut acceleration,
		force,
		resistance
	) in query.iter_mut() {
		let delta = time.delta_seconds();
		// Force -> Acceleration
		acceleration.linear  += force.linear  / resistance.mass   ;
		acceleration.angular += force.angular / resistance.inertia;
		// Acceleration -> Velocity
		velocity.linear  += acceleration.linear  * delta;
		velocity.angular += acceleration.angular * delta;
		// Velocity -> Transform
		transform.translation += velocity.linear.extend(0.0) * delta;
		transform.rotate(Quat::from_rotation_z(velocity.angular * delta));
	}
}
