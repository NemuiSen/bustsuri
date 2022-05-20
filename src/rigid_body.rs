use bevy::prelude::*;

use crate::prelude::Kinematic;

#[derive(Bundle)]
pub struct RigidBodyBundle {
	body_type: BodyType,
	#[bundle]
	kinematic: Kinematic,
}

#[derive(Component)]
pub enum BodyType {
	Dynamic,
	Static,
}

