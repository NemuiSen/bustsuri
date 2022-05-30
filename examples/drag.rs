use std::f32::consts::PI;
use bevy::prelude::*;
use butsuri::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(PhysicsPlugin::default())
		.add_startup_system(setup)
		.add_system(move_shape)
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
	.insert_bundle(KinematicBundle {
		forces: ForcesBundle {
			drag: Drag {
				linear: 0.01,
				angular: 0.01,
			},
			..default()
		},
		..default()
	})
	.insert(Player);
}

#[derive(Component)]
struct Player;
fn move_shape(
	input: Res<Input<KeyCode>>,
	mut query: Query<&mut Force, With<Player>>
) {
	let mut force = query.single_mut();
	let mut delta = Vec2::ZERO;
	if input.pressed(KeyCode::W) { delta.y += 1. }
	if input.pressed(KeyCode::A) { delta.x -= 1. }
	if input.pressed(KeyCode::S) { delta.y -= 1. }
	if input.pressed(KeyCode::D) { delta.x += 1. }
	force.linear = delta.normalize_or_zero() * 100.;

	let mut delta = 0.0;
	if input.pressed(KeyCode::Q) { delta -= PI }
	if input.pressed(KeyCode::E) { delta += PI }
	force.angular = delta;
}

