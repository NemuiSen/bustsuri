use bevy::{
	prelude::*,
	math::{vec3, vec2}
};

use butsuri::prelude::*;
use rand::{Rng, thread_rng};

const MAX_SHAPE_SIZE: f32 = 50.0;
const MIN_SHAPE_SIZE: f32 = 25.0;
const SPRITE_SIZE: f32 = 25.0;

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

	for _ in 0..25 {
		commands.spawn_bundle(SpriteBundle {
				sprite: Sprite {
					custom_size: Some(Vec2::splat(SPRITE_SIZE)),
					color: Color::TOMATO,
					..Default::default()
				},
				transform: Transform::from_translation(random_position()),
				..Default::default()
			})
		.insert_bundle(random_collier_shape())
		.insert_bundle(Dynamic {
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

fn random_collier_shape() -> ColliderBundle {
	let mut collider = ColliderBundle::default();
	match thread_rng().gen::<bool>() {
		true  => collider.shape = {
			let w = thread_rng().gen_range(MIN_SHAPE_SIZE..=MAX_SHAPE_SIZE);
			let h = thread_rng().gen_range(MIN_SHAPE_SIZE..=MAX_SHAPE_SIZE);
			ColliderShape::AABB(Vec2::new(w, h))
		},
		false => collider.shape = {
			let r = thread_rng().gen_range(MIN_SHAPE_SIZE..=MAX_SHAPE_SIZE);
			ColliderShape::Circle(r)
		},
	} 
	collider
}

fn out_of_bounds(
	mut query: Query<&mut Transform>,
	windows: Res<Windows>
) {
	let window = windows.get_primary().unwrap();
	let width = window.requested_width();
	let height = window.requested_height();
	let hlimit = width /2.0;
	let vlimit = height/2.0;
	for mut transform in query.iter_mut() {
		if transform.translation.x < -hlimit-1.0 { transform.translation.x =  hlimit; }
		if transform.translation.x >  hlimit+1.0 { transform.translation.x = -hlimit; }
		if transform.translation.y < -vlimit-1.0 { transform.translation.y =  vlimit; }
		if transform.translation.y >  vlimit+1.0 { transform.translation.y = -vlimit; }
	}
}

fn collision_reaction (
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

