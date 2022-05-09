use std::f32::consts::PI;

use bevy::{
	prelude::*,
	math::*
};
use butsuri::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(PhysicsPlugin::default())
		.add_plugin(DebugPlugin)
		.add_startup_system(setup)
		.add_system(move_shape)
		.add_system(check_collision)
	.run();
}

fn setup(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	commands.spawn_bundle(SpriteBundle {
		sprite: Sprite {
			custom_size: Some(Vec2::splat(25.)),
			color: Color::TURQUOISE,
			..Default::default()
		},
		..Default::default()
	})
	.insert_bundle(Kinematic::default())
	.insert(ColliderShape::AABB(25., 25.))
	.insert(Player);

	commands.spawn_bundle(SpriteBundle {
		sprite: Sprite {
			custom_size: Some(Vec2::splat(50.)),
			..Default::default()
		},
		transform: Transform::from_translation(vec3( 200., 0., 0.)),
		..Default::default()
	})
	.insert(ColliderShape::AABB(50., 50.));

	commands.spawn_bundle(SpriteBundle {
		sprite: Sprite {
			custom_size: Some(Vec2::splat(50.)),
			..Default::default()
		},
		transform: Transform::from_translation(vec3(-200., 0., 0.)),
		..Default::default()
	})
	.insert(ColliderShape::Circle(50.));
}

fn check_collision(
	mut collision_event: EventReader<CollisionEvent>,
	mut query: Query<&mut Sprite>,
) {
	for mut sprite in query.iter_mut() {
		sprite.color = Color::BLUE;
	}
	for &CollisionEvent(dst, _src) in collision_event.iter() {
		query.get_mut(dst).unwrap().color = Color::RED;
	}
}

#[derive(Component)]
struct Player;
fn move_shape(
	input: Res<Input<KeyCode>>,
	mut query: Query<&mut Velocity, With<Player>>
) {
	let mut velocity = query.single_mut();
	let mut delta = Vec2::ZERO;
	if input.pressed(KeyCode::W) { delta.y += 1. }
	if input.pressed(KeyCode::A) { delta.x -= 1. }
	if input.pressed(KeyCode::S) { delta.y -= 1. }
	if input.pressed(KeyCode::D) { delta.x += 1. }
	velocity.linear = delta.normalize_or_zero() * 100.;
	
	let mut delta = 0.0;
	if input.pressed(KeyCode::Q) { delta -= PI }
	if input.pressed(KeyCode::E) { delta += PI }
	velocity.angular = delta;
}

