mod components;
mod systems;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use components::*;
use systems::*;

fn main() {
    App::new()
        // ↓ Bevy
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Bevy - My testing app ^_^".into(),
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), // prevents blurry sprites
        )
        // ↓ Rapier
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default()) // draws borders around colliders
        // ↓ LDtk
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        // ↓ Setup
        .add_startup_system(setup)
        .add_startup_system(setup_ui)
        .add_plugin(SpawnPlugin)
        // ↓ Run
        .add_system(move_player)
        .add_system(move_camera)
        .add_system(animate_player)
        .add_system(player_derive_z_from_y)
        .add_system(collision_events)
        .run();
}

struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("EntityPlayer")
            .register_ldtk_entity::<SignBundle>("EntitySign")
            .register_ldtk_entity::<DoorBundle>("EntityDoor")
            .register_ldtk_int_cell::<WallBundle>(1)
            .add_system(spawn_player)
            .add_system(spawn_sign)
            .add_system(spawn_door)
            .add_system(spawn_wall_collision)
            .add_system(spawn_derive_z_from_y);
    }
}

#[derive(Bundle)]
struct MySpriteBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn({
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.5,
                ..default()
            },
            ..default()
        }
    });

    // Ldtk world
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("LDtk/world1.ldtk"),
        ..default()
    });

    // Test sprites
    commands
        .spawn(MySpriteBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.7, 0.7, 0.7),
                    custom_size: Some(Vec2::new(200.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0., -100., 0.)),
                ..default()
            },
            collider: Collider::cuboid(100.0, 25.0),
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(DoorRef {
            target_level_iid: LevelSelection::Iid(
                "cb1238d0-5110-11ed-b5f8-774d9bde9a1c".to_owned(),
            ),
            target_entity_iid: EntityIid("4618b090-7820-11ed-8a70-c9c419731504".to_owned()),
        });

    commands.spawn(MySpriteBundle {
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.1),
                custom_size: Some(Vec2::new(50.0, 200.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(200., 0., 0.)),
            ..default()
        },
        collider: Collider::cuboid(25.0, 100.0),
    });
}
