use bevy::prelude::*;
use crate::prelude::*;

#[derive(Default, Bundle)]
pub struct RigidBodyBundle {
	#[bundle] pub kinematic: KinematicBundle,
	#[bundle] pub collider: ColliderBundle,
	pub body: Body,
	pub is_sleep: IsSleep,
}

#[derive(Default, Component, PartialEq)]
pub enum Body {
	Static,
	#[default]
	Dynamic,
}

#[derive(Default, Component, Deref, DerefMut)]
pub struct IsSleep(bool);

pub(crate) fn is_sleep(
	mut query: Query<(&Velocity, &Acceleration, &Force, &mut IsSleep)>
) {
	for (velocity, acceleration, force, mut is_sleep) in query.iter_mut() {
		let sleep =
			tend_zero_kinematic(velocity.linear, velocity.angular) ||
			tend_zero_kinematic(acceleration.linear, acceleration.angular) ||
			tend_zero_kinematic(force.linear, force.angular);
		if **is_sleep != sleep { **is_sleep = sleep; }
	}
}

#[inline]
fn tend_zero_kinematic(l: Vec2, a: f32) -> bool {
	l.x.abs() < f32::EPSILON || l.y.abs() < f32::EPSILON || a.abs() < f32::EPSILON
}

