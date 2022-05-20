pub mod kinematic;
pub mod collision;
pub mod rigid_body;

pub mod prelude {
	pub use crate::kinematic::*;
	pub use crate::collision::*;
	pub use crate::PhysicsPlugin;
	pub use crate::GravityEffect;
	pub use crate::DebugPlugin;
	pub use crate::Gravity;
}

use bevy::{prelude::*, math::vec2};

#[derive(Default)]
pub enum GravityEffect {
	#[default]
	None,
	Velocity,
	Acceleration,
}

#[derive(Default)]
pub struct PhysicsPlugin {
	pub gravity_effect: GravityEffect,
}

impl Plugin for PhysicsPlugin {
	fn build(&self, app: &mut App) {
		let mut post_update = SystemSet::new().with_system(kinematic::update_transform);
		post_update = match self.gravity_effect {
			GravityEffect::None         => post_update,
			GravityEffect::Velocity     => post_update.with_system(gravity_velocity    .before(kinematic::update_transform)),
			GravityEffect::Acceleration => post_update.with_system(gravity_acceleration.before(kinematic::update_transform))
		};

		app
			.add_system_to_stage(CoreStage::PreUpdate, collision::collision_info)
			.add_system_set_to_stage(
				CoreStage::PostUpdate,
				post_update
			);
	}
}

#[derive(Deref, DerefMut)]
pub struct Gravity(pub Vec2);

fn gravity_acceleration(
	gravity: Option<Res<Gravity>>,
	mut query: Query<&mut kinematic::Acceleration>,
) {
	if let Some(g) = gravity {
		for mut acceleration in query.iter_mut() {
			acceleration.linear += **g;
		}
	}
}

fn gravity_velocity(
	gravity: Option<Res<Gravity>>,
	mut query: Query<&mut kinematic::Velocity>
) {
	if let Some(g) = gravity {
		for mut velocity in query.iter_mut() {
			velocity.linear += **g;
		}
	}
}

// Debug Plugin

use bevy_prototype_lyon::prelude::*;
use collision::*;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
	fn build(&self, app: &mut App) {
	    app.add_plugin(ShapePlugin)
			.add_system(spawn_debug_shape)
			.add_system(collider_debug_transform_sync);
	}
}

#[derive(Component)] struct ColliderDebugParent(Entity);
#[derive(Component)] struct ColliderDebugChild(Entity);

fn spawn_debug_shape(
	mut commands: Commands,
	query: Query<(Entity, &ColliderShape), Added<ColliderShape>>
) {
	for (parent, collider_shape) in query.iter() {
		let mut builder = GeometryBuilder::new();
		match collider_shape {
			ColliderShape::AABB(size) => {
				builder = builder.add(&shapes::Rectangle {
					extents: vec2(size.x*2.0, size.y*2.0),
					..default()
				});
			},
			ColliderShape::Circle(r) => {
				builder = builder.add(&shapes::Circle {
					radius: *r,
					..default()
				});
			},
			_ => unreachable!()
		}

		let child = commands.spawn_bundle(builder.build(
			DrawMode::Stroke(StrokeMode::color(Color::GREEN)),
			default()
		)).insert(ColliderDebugParent(parent)).id();
	
		commands.entity(parent).insert(ColliderDebugChild(child));
	}
}

fn collider_debug_transform_sync(
	mut commands: Commands,
	mut child_query: Query<(Entity, &mut Transform, &ColliderDebugParent)>,
	parent_query: Query<(&Transform, &ColliderShape) , (With<ColliderDebugChild>, Without<ColliderDebugParent>)>,
) {
	for (child, mut child_transform, ColliderDebugParent(parent)) in child_query.iter_mut() {
		if let Ok((parent_transform, collider_shape)) = parent_query.get(*parent) {
			match collider_shape {
				ColliderShape::AABB(_) => child_transform.translation = parent_transform.translation,
				_ => *child_transform = *parent_transform
			}
			
		} else {
			commands.entity(child).despawn();
		}
	}
}

