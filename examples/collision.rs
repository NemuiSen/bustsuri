use std::f32::consts::{TAU, PI};

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
		.insert_bundle(ColliderBundle::new(random_collier_shape()))
		.insert_bundle(DynamicBundle {
			velocity: random_velocity(),
			..Default::default()
		});
	}
}

fn input(
	input: Res<Input<KeyCode>>,
	mut query: Query<(&mut Transform, &mut Velocity, &mut ColliderShape)>
) {
	for (mut transform, mut velocity, mut shape) in query.iter_mut() {
		if input.just_pressed(KeyCode::P) {
			transform.translation = random_position();
		}
		if input.just_pressed(KeyCode::V) {
			*velocity = random_velocity();
		}
		if input.just_pressed(KeyCode::S) {
			*shape = random_collier_shape();
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

fn random_velocity() -> Velocity {
	Velocity {
		linear: vec2(
			thread_rng().gen_range(-1. ..= 1.),
			thread_rng().gen_range(-1. ..= 1.)
		).normalize_or_zero() * thread_rng().gen_range(25. ..= 50.),
		angular: thread_rng().gen_range(rad(-22.5)..=rad(22.5)),
	}
}

fn rad(deg: f32) -> f32 { deg*PI/180. }

fn random_collier_shape() -> ColliderShape {
	match thread_rng().gen_range(0u8..=3) {
		0 => {
			let w = thread_rng().gen_range(MIN_SHAPE_SIZE..=MAX_SHAPE_SIZE);
			let h = thread_rng().gen_range(MIN_SHAPE_SIZE..=MAX_SHAPE_SIZE);
			ColliderShape::Square(w, h)
		},
		1 => {
			let r = thread_rng().gen_range(MIN_SHAPE_SIZE..=MAX_SHAPE_SIZE);
			ColliderShape::Circle(r)
		},
		_ => {
			let count = thread_rng().gen_range(3u8..=10);
			let offset = TAU/(count as f32);
			let vertices = (0..count).map(|i| {
				let radius = thread_rng().gen_range(MIN_SHAPE_SIZE..=MAX_SHAPE_SIZE);
				let angle = i as f32 * offset;
				vec2(
					angle.cos() * radius,
					angle.sin() * radius,
				)
			}).collect();
			ColliderShape::Polygon(vertices)
		}
	}
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
		if transform.translation.x < -hlimit { transform.translation.x =  hlimit; }
		if transform.translation.x >  hlimit { transform.translation.x = -hlimit; }
		if transform.translation.y < -vlimit { transform.translation.y =  vlimit; }
		if transform.translation.y >  vlimit { transform.translation.y = -vlimit; }
	}
}

fn collision_reaction (
	mut query: Query<(&mut Sprite, &CollisionInfo)>,
) {
	for (mut sprite, info) in query.iter_mut() {
		if info.is_colliding {
			sprite.color = Color::RED;
		} else {
			sprite.color = Color::BLUE;
		}
	}
}

