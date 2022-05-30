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
	.insert_bundle(RigidBodyBundle {
		collider: ColliderBundle::new(ColliderShape::Square(25.0, 25.0)),
		..default()
	})
	.remove_bundle::<ForcesBundle>()
	.insert(Player);

	commands.spawn_bundle(SpriteBundle {
		sprite: Sprite {
			custom_size: Some(Vec2::splat(50.)),
			..Default::default()
		},
		transform: Transform::from_translation(vec3( 200., 0., 0.)).with_rotation(Quat::from_rotation_z(PI/4.0)),
		..Default::default()
	})
	.insert_bundle(ColliderBundle::new(ColliderShape::Square(50.0, 50.0)));

	commands.spawn_bundle(SpriteBundle {
		sprite: Sprite {
			custom_size: Some(Vec2::splat(50.)),
			..Default::default()
		},
		transform: Transform::from_translation(vec3(-200., 0., 0.)),
		..Default::default()
	})
	.insert_bundle(ColliderBundle::new(ColliderShape::Circle(50.0)));
}

fn check_collision(
	mut query: Query<(&mut Sprite, &CollisionInfo)>,
) {
	for (mut sprite, info) in query.iter_mut() {
		if info.is_colliding {
			sprite.color = Color::BLUE;
		} else {
			sprite.color = Color::RED;
		}
	}
}

#[derive(Component)]
struct Player;
fn move_shape(
	input: Res<Input<KeyCode>>,
	mut query: Query<(&mut Velocity, &mut Body), With<Player>>
) {
	let (mut velocity, mut is_static) = query.single_mut();
	if input.just_pressed(KeyCode::P) {
		*is_static = match *is_static {
			Body::Static  => Body::Dynamic,
			Body::Dynamic => Body::Static,
		};
	}
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

