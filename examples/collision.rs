use bevy::{
	prelude::*,
	math::{vec3, vec2, const_vec2}
};

use butsuri::prelude::*;
use rand::{Rng, thread_rng};

const COLLIDER_SHAPE_SIZE: f32 = 25.;
const AREA_SIZE: Vec2 = const_vec2!([300.; 2]);

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(PhysicsPlugin::default())
		.add_plugin(DebugPlugin)
		.add_startup_system(setup)
		.add_system(input)
		.add_system(out_of_bounds)
		.add_system(collision_reaction)
		.run();
}

fn setup(
	mut commands: Commands
) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	for _ in 0..20 {
		commands.spawn_bundle(SpriteBundle {
				sprite: Sprite {
					custom_size: Some(Vec2::splat(COLLIDER_SHAPE_SIZE)),
					color: Color::TOMATO,
					..Default::default()
				},
				transform: Transform::from_translation(random_position()),
				..Default::default()
			})
		.insert(random_collier_shape())
		.insert_bundle(Kinematic {
			velocity: Velocity {
				linear: random_velocity(),
				angular: 0.0,
			},
			..Default::default()
		});
	}
}

fn input(input: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &mut Velocity)>) {
	if input.just_pressed(KeyCode::P) {
		for (mut transform, _) in query.iter_mut() {
			transform.translation = random_position();
		}
	}
	if input.just_pressed(KeyCode::V) {
		for (_, mut velocity) in query.iter_mut() {
			velocity.linear = random_velocity();
		}
	}
}

fn random_position() -> Vec3 {
	vec3(
		thread_rng().gen_range(-100.0..=100.0),
		thread_rng().gen_range(-100.0..=100.0),
		0.0,
	)
}

fn random_velocity() -> Vec2 {
	vec2(
		thread_rng().gen_range(-1.0..=1.0),
		thread_rng().gen_range(-1.0..=1.0),
	).normalize() * thread_rng().gen_range(25. ..= 50.)
}

fn random_collier_shape() -> ColliderShape {
	match thread_rng().gen::<bool>() {
		true  => ColliderShape::AABB(COLLIDER_SHAPE_SIZE, COLLIDER_SHAPE_SIZE),
		false => ColliderShape::Circle(COLLIDER_SHAPE_SIZE),
	} 
}

fn out_of_bounds(mut query: Query<(&mut Transform, &mut Velocity)>) {
	for (mut transform, mut velocity) in query.iter_mut() {
		let horizontal = transform.translation.x > AREA_SIZE.x || transform.translation.x < -AREA_SIZE.x;
		let vertical   = transform.translation.y > AREA_SIZE.x || transform.translation.y < -AREA_SIZE.y; 
		if horizontal || vertical {
			velocity.linear = random_velocity();
			transform.translation = random_position();
		}
	}
}

fn collision_reaction (
	mut collision_event: EventReader<CollisionEvent>,
	mut query: Query<&mut Sprite>,
) {
	for mut sprite in query.iter_mut() {
		sprite.color = Color::BLUE
	}
	for &CollisionEvent(dst, _src) in collision_event.iter() {
		query.get_mut(dst).unwrap().color = Color::RED;
	}
}

