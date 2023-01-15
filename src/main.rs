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
        // .add_plugin(RapierDebugRenderPlugin::default()) // draws borders around colliders
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
        .add_system(animation)
        .add_system(derive_z_from_y_after_move)
        .add_system(collision_events)
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
            .add_system(Player::spawn)
            .add_system(Sign::spawn)
            .add_system(Door::spawn)
            .add_system(ToriiGate::spawn)
            .add_system(TreeBig::spawn)
            .add_system(TreeSmall::spawn)
            .add_system(Wall::spawn)
            .add_system(DeriveZFromY::spawn);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn({
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.3,
                ..default()
            },
            ..default()
        }
    });

    // Ldtk world
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("LDtk/world2.ldtk"),
        ..default()
    });
}
