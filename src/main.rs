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
                    primary_window: Some(Window {
                        title: "Bevy - My testing app ^_^".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), // prevents blurry sprites
        )
        // ↓ Rapier
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default()) // draws borders around colliders
        // ↓ LDtk
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        // ↓ Setup
        .add_systems(Startup, (setup, setup_ui))
        .add_plugins(SpawnPlugin)
        // ↓ Run
        .add_systems(
            Update,
            (
                move_player,
                move_camera,
                animation,
                derive_z_from_y_after_move,
                collision_events,
            ),
        )
        .run();
}

struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<SignBundle>("Sign")
            .register_ldtk_entity::<DoorBundle>("Door")
            .register_ldtk_entity::<FrogBundle>("Frog")
            .register_ldtk_entity::<HouseBundle>("House")
            .register_ldtk_entity::<ToriiGateBundle>("ToriiGate")
            .register_ldtk_entity::<TreeBigBundle>("TreeBig")
            .register_ldtk_entity::<TreeSmallBundle>("TreeSmall")
            .register_ldtk_int_cell::<WallBundle>(4) // higher ground
            .register_ldtk_int_cell::<WallBundle>(5) // fences
            .register_ldtk_int_cell::<WallBundle>(6) // thick trees
            .register_ldtk_int_cell::<WallBundle>(8) // rocks
            .register_ldtk_int_cell::<WallBundle>(9) // invisible walls
            .register_ldtk_int_cell::<WallBundle>(10) // interior walls
            .add_systems(Update, Player::spawn)
            .add_systems(Update, Sign::spawn)
            .add_systems(Update, Door::spawn)
            .add_systems(Update, ToriiGate::spawn)
            .add_systems(Update, TreeBig::spawn)
            .add_systems(Update, TreeSmall::spawn)
            .add_systems(Update, Wall::spawn)
            .add_systems(Update, DeriveZFromY::spawn);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3 {
            x: 0.35,
            y: 0.35,
            z: 1.,
        }),
        ..default()
    });

    // Ldtk world
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("LDtk/world2.ldtk"),
        ..default()
    });
}
