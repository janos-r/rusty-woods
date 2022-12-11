mod components;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use components::*;

fn main() {
    App::new()
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(LevelSelection::Index(0))
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
        .add_plugin(LdtkPlugin)
        .register_ldtk_entity::<components::player::PlayerBundle>("EntityPlayer")
        .register_ldtk_entity::<SignBundle>("EntitySign")
        .register_ldtk_entity::<DoorBundle>("EntityDoor")
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default()) // draws borders around colliders
        .add_system(components::player::spawn_player)
        .add_startup_system(setup)
        .add_startup_system(setup_ui)
        .add_system(spawn_sign)
        .add_system(move_player)
        .add_system(move_camera)
        .add_system(animate_sprite_system_velocity)
        .add_system(collision_events)
        .run();
}

#[derive(Bundle)]
struct MySpriteBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Ldtk world
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("LDtk/world1.ldtk"),
        ..default()
    });

    // Sprites
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
        .insert(Destination {
            level: "Level_1".to_owned(),
            coords: Vec3::new(0., 100., 3.),
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

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    if let Ok(mut player_velocity) = query.get_single_mut() {
        const SPEED: f32 = 200.;

        let default = Vect::default();
        if player_velocity.linvel != default {
            player_velocity.linvel = default;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            player_velocity.linvel += Vect::new(-SPEED, 0.);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            player_velocity.linvel += Vect::new(SPEED, 0.);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            player_velocity.linvel += Vect::new(0., SPEED);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            player_velocity.linvel += Vect::new(0., -SPEED);
        }
    }
}

fn move_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}

fn animate_sprite_system_velocity(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut components::AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &Velocity,
        ),
        Changed<Velocity>,
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle, velocity) in &mut query {
        timer.tick(time.delta());
        if velocity.linvel == Vect::default() {
            sprite.index = 0;
        } else if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    // ↓ Doors
    mut player_query: Query<&mut Transform, With<Player>>,
    mut level: ResMut<LevelSelection>,
    destination_query: Query<&Destination>,
    // ↓ Signs
    sign_collider_query: Query<&Parent, With<SignCollider>>,
    sign_query: Query<&SignText>,
    text_box_query: Query<(Entity, &Children, &Handle<Font>), With<TextBox>>,
    mut commands: Commands,
    mut text_box_visibility: Query<&mut Visibility, With<TextBoxContainer>>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            // lets not hope that the door will always be e1 - lets try both and also stop if it was first
            for entity in [e1, e2] {
                if let Ok(destination) = destination_query.get(*entity) {
                    // door - switch_level
                    if let Ok(mut player_transform) = player_query.get_single_mut() {
                        *level = LevelSelection::Identifier(destination.level.clone());
                        player_transform.translation = destination.coords;
                    }
                    break;
                } else if let Ok(parent) = sign_collider_query.get(*entity) {
                    // sign - display text
                    if let Ok(text) = sign_query.get(parent.get()) {
                        // clear text
                        // despawning children (here the words)
                        // issue: https://bevy-cheatbook.github.io/features/parent-child.html?highlight=remove_chil#despawning-child-entities
                        let (entity, children, font_handle) = text_box_query.single();
                        commands.entity(entity).remove_children(children);
                        for child in children {
                            commands.entity(*child).despawn_recursive();
                        }
                        // open text_box
                        text_box_visibility.single_mut().is_visible = true;
                        // new text
                        commands.entity(entity).add_children(spawn_children_text(
                            font_handle.clone(),
                            text.0.to_owned(),
                        ));
                        break;
                    }
                };
            }
        } else if let CollisionEvent::Stopped(e1, e2, _) = collision_event {
            for entity in [e1, e2] {
                if sign_collider_query.contains(*entity) {
                    // close text_box
                    text_box_visibility.single_mut().is_visible = false;
                    break;
                }
            }
        }
    }
}
