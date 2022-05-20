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

/// a=v*t
/// whatever value "Acceleration" has will be added to "Velocity"
#[derive(Default, Component)]
pub struct Acceleration {
	/// Affects the position
	pub linear: Vec2,
	/// Affects the rotation
	pub angular: f32,
}

/// F=m*a
/// Any value that "Force" takes will be applied directly to "Acceleration" overwriting any previous value it had
#[derive(Default, Component)]
pub struct Force {
	/// Affects the position
	pub linear: Vec2,
	/// Affects the rotation
	pub angular: f32,
}

/// Object resistance
#[derive(Component)]
pub struct Resistance {
	/// Object linear resistance
	pub mass: f32,
	/// Object angular resistance
	pub inertia: f32,
}

/// Enviroment resistance
#[derive(Default, Component)]
pub struct Drag {
	pub linear: f32,
	pub angular: f32,
}

impl Default for Resistance {
	fn default() -> Self {
	    Self {
			mass: 1.,
			inertia: 1.,
		}
	}
}

#[derive(Default, Bundle)]
pub struct Dynamic {
	pub velocity: Velocity,
	pub acceleration: Acceleration,
}

#[derive(Default, Bundle)]
pub struct Kinematic {
	pub velocity: Velocity,
	pub acceleration: Acceleration,
	pub force: Force,
	pub resistance: Resistance,
	pub drag: Drag,
}

pub(crate) fn update_transform (
	time: Res<Time>,
	mut query: Query<(
		&mut Transform,
		&mut Velocity,
		&mut Acceleration,
		Option<&mut Force>,
		Option<&Resistance>,
		Option<&Drag>,
	)>
) {
	for (
		mut transform,
		mut velocity,
		mut acceleration,
		mut force,
		resistance,
		drag,
	) in query.iter_mut() {
		let delta = time.delta_seconds();
		// Drag -> Force
		if let Some((drag, force)) = drag.zip(force.as_mut()) {
			let  linear_speed_speed = velocity.linear.length_squared();
			let angular_speed_speed = velocity.angular * velocity.angular;
			let  linear_drag_magnitude = drag.linear  *  linear_speed_speed;
			let angular_drag_magnitude = drag.angular * angular_speed_speed;
			let  linear_drag = -velocity.linear.normalize_or_zero() * linear_drag_magnitude;
			let angular_drag = -velocity.angular * angular_drag_magnitude;
			force.linear  += linear_drag;
			force.angular += angular_drag;
		}
		// Force -> Acceleration
		if let Some((force, resistance)) = force.zip(resistance) {
			acceleration.linear  = force.linear  / resistance.mass   ;
			acceleration.angular = force.angular / resistance.inertia;
		}
		// Acceleration -> Velocity
		velocity.linear  += acceleration.linear  * delta;
		velocity.angular += acceleration.angular * delta;
		// Velocity -> Transform
		transform.translation += velocity.linear.extend(0.0) * delta;
		transform.rotate(Quat::from_rotation_z(velocity.angular * delta));
	}
}

