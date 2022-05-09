use bevy::{prelude::*, math::vec2};
use butsuri::prelude::*;
use rand::{thread_rng, Rng};

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(PhysicsPlugin {
			gravity_effect: GravityEffect::Velocity,
		})
		.insert_resource(Gravity(Vec2::Y * -9.8))
		.add_startup_system(setup)
		.add_system(input)
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	commands.spawn_bundle(SpriteBundle {
		sprite: Sprite {
			custom_size: Some(vec2(50.0, 50.0)),
			color: Color::CRIMSON,
			..Default::default()
		},
		..Default::default()
	})
	.insert_bundle(Kinematic::default());
}

fn input(
	keyboard: Res<Input<KeyCode>>,
	mut gravity: ResMut<Gravity>,
	mut query: Query<(&mut Transform, &mut Velocity)>
) {
	let (mut transform, mut velocity) = query.single_mut();
	let mut delta = Vec2::ZERO;
	if keyboard.pressed(KeyCode::W) { delta.y += 1. }
	if keyboard.pressed(KeyCode::S) { delta.y -= 1. }
	if keyboard.pressed(KeyCode::D) { delta.x += 1. }
	if keyboard.pressed(KeyCode::A) { delta.x -= 1. }
	velocity.linear = delta.normalize_or_zero() * 100.;

	if keyboard.just_pressed(KeyCode::R) {
		velocity.linear = Vec2::ZERO;
		transform.translation = Vec3::ZERO;
		**gravity = vec2(
			thread_rng().gen_range(-100. ..= 100.),
			thread_rng().gen_range(-100. ..= 100.),
		)
	}
}

