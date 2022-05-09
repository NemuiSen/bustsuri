use bevy::prelude::*;

#[derive(Clone, Component)]
pub enum BodyType {
	Dynamic,
	Static,
}

