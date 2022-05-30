use bevy::prelude::*;

use crate::prelude::KinematicBundle;
use crate::prelude::ColliderBundle;

#[derive(Default, Bundle)]
pub struct RigidBodyBundle {
	#[bundle] pub kinematic: KinematicBundle,
	#[bundle] pub collider: ColliderBundle,
	pub is_static: IsStatic,
	pub is_sleep: IsSleep,
}

#[derive(Default, Component, Deref, DerefMut)] pub struct IsStatic(bool);
#[derive(Default, Component, Deref, DerefMut)] pub struct IsSleep(bool);

