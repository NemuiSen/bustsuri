# Busturi
[![latest version](https://img.shields.io/crates/v/butsuri)](https://crates.io/crates/butsuri)

~~A physics engine~~ An asset that provides 2d collision detector and kinematics, build from scratch in [bevy](bevyengine.org)

### How to use
Add PhysicsPlugin to the program and optionally also DebugPlugin to see the collisions of the entities.
```rust, no_run
App::new()
	.add_plugins(DefaultPlugins)
	.add_plugin(PhysicsPlugin::default())
	// This plugin allows us to see the collisions
	//.add_plugin(DebugPlugin)
```

Then after creating an entity with at least the Transform component insert the CollisionShape component to detect the collisions and who it collided with, and if you want insert the KinematicBundle that the entity has physical properties such as speed, acceleration, or strength
```rust, no_run
commands.spawn_bundle(SpriteBundle {
	sprite: Sprite {
		custom_size: Some(Vec2::splat(100.)),
		color: Color::CYAN,
		..Default::default()
	},
	..Default::default()
})
// The size you pass to ColliderShape::AABB(_, _) should be half what you want
.insert(ColliderShape::AABB(50., 50.));
```

In the end it should be something similar to the following example
```rust, no_run
use bevy::prelude::*;
use butsuri::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(PhysicsPlugin::default())
		.add_system(setup)
	.run();
}

fn setup(mut commands: Commands) {
	commands.spawn_bundle(SpriteBundle {
		sprite: Sprite {
			custom_size: Some(Vec2::splat(100.)),
			color: Color::CYAN,
			..Default::default()
		},
		..Default::default()
	})
	.insert(ColliderShape::AABB(50., 50.));
}

// handle collisions as you want
fn collision(
	mut collision_event: EventReader<CollisionEvent>,
	mut query: Query<&mut Sprite>,
) {
	// ...
}

```

### Examples
- [Collision](./examples/collision.rs): a group of sprites that change color when they collide
- [Player Input](./examples/player_input.rs): similar to the previous example but there is a player that is controlled by applying velocity with the w/a/s/d keys
- [Gravity](./examples/gravity.rs): shows how to activate and use gravity

### How useful is this project compared to other similar ones?
Little, because there are better options that offer more features for example [heron](https://crates.io/crates/heron) or [bevy_rapier](https://github.com/dimforge/bevy_rapier). I did this project to learn how physics engines work from the inside and i'm a bit lost with things like momentum, but i'll keep updating this project to improve my knowledge, which I'm open to criticism or recommendations.

if you want to contact me you can do it by discord (`NemuiSen#4114`) on the Bevy server

PS: I'm sorry if something doesn't make sense, I'm using my vague knowledge of English and google translate to write this.
