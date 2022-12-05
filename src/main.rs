use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

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
        .register_ldtk_entity::<PlayerBundle>("EntityPlayer")
        .register_ldtk_entity::<SignBundle>("EntitySign")
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default()) // draws borders around colliders
        .add_startup_system(setup)
        .add_system(spawn_player)
        .add_system(spawn_sign)
        .add_system(move_player)
        .add_system(move_camera)
        .add_system(animate_sprite_system_velocity)
        .add_system(collision_events)
        .run();
}

#[derive(Component, Default)]
struct Player;

#[derive(Component, Default, Deref, DerefMut)]
struct AnimationTimer(Timer);

// Text box
#[derive(Component)]
struct TextBoxContainer;
#[derive(Component)]
struct TextBox;

#[derive(Component)]
struct Destination {
    level: usize,
    x: f32,
    y: f32,
}

// Local Bundles
#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    sprite_sheet_bundle: SpriteSheetBundle,
    velocity: Velocity,
    #[worldly]
    worldly: Worldly,
}

fn spawn_player(
    mut commands: Commands,
    query_player: Query<Entity, Added<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if let Ok(player) = query_player.get_single() {
        let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .entity(player)
            .insert(AnimationTimer(Timer::from_seconds(
                0.08,
                TimerMode::Repeating,
            )))
            .insert(texture_atlas_handle)
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::ROTATION_LOCKED)
            // Position the collider relative to the rigid-body.
            .with_children(|parent| {
                parent.spawn((
                    TransformBundle::from(Transform::from_xyz(0., -8., 0.)),
                    Collider::ball(8.),
                ));
            });
    }
}

#[derive(Component, Default)]
struct Sign;

#[derive(Component, Default)]
struct SignCollider;

#[derive(Component, Default)]
struct Text(String);

#[derive(Bundle, LdtkEntity)]
struct SignBundle {
    sign: Sign,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    text: Text,
}

impl From<EntityInstance> for Text {
    fn from(entity_instance: EntityInstance) -> Text {
        if let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "String")
        {
            if let FieldValue::String(Some(text)) = &field_instance.value {
                Text(text.to_owned())
            } else {
                default()
            }
        } else {
            default()
        }
    }
}

fn spawn_sign(mut commands: Commands, query_sign: Query<Entity, Added<Sign>>) {
    for sign in &query_sign {
        commands.entity(sign).with_children(|parent| {
            parent.spawn((
                SignCollider,
                // Position the collider relative to the rigid-body.
                TransformBundle::from(Transform::from_xyz(0., -3., 0.)),
                Collider::ball(8.),
                ActiveEvents::COLLISION_EVENTS,
            ));
        });
    }
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
            level: 1,
            x: 0.,
            y: 100.,
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

    // Bottom text box
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(200.0)),
            // align container to the bottom
            align_self: AlignSelf::FlexEnd,
            // makes space bellow the box
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            ..default()
        },
        // transparent container
        background_color: Color::rgba(0.65, 0.65, 0.65, 0.).into(),
        ..default()
    })
    .insert(TextBoxContainer)
    .with_children(|parent| {
        // box size, border thickness and color
        parent.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.), Val::Percent(80.0)),
                border: UiRect::all(Val::Px(6.0)),
                ..default()
            },
            background_color: Color::MIDNIGHT_BLUE.into(),
            ..default()
        })
        .with_children(|parent| {
            // text background
            let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.)),
                    padding: UiRect::all(Val::Px(6.)),
                    // don't stretch vertically
                    align_content: AlignContent::FlexStart,
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            })
            .insert(TextBox)
            .insert(font_handle.clone())
            .with_children(spawn_children_text(font_handle, String::from(
                // "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularized in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."
                "Text Example a little longer trying to cross the width. Text Example a little longer trying to cross the width"
            )));
        });
    });
}

fn spawn_children_text(font_handle: Handle<Font>, text: String) -> impl FnOnce(&mut ChildBuilder) {
    // text wrapping solution (bug workaround) based on: https://github.com/bevyengine/bevy/issues/1490
    const FONT_SIZE: f32 = 30.;
    move |parent: &mut ChildBuilder| {
        // "Text Example",
        for word in text.split_whitespace() {
            parent.spawn(
                TextBundle::from_section(
                    word.to_string(),
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: FONT_SIZE,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    // this is required because of the bevy bug https://github.com/bevyengine/bevy/issues/5834
                    max_size: Size::new(Val::Undefined, Val::Px(FONT_SIZE)),
                    // this is the size of the spaces between words
                    margin: UiRect::all(Val::Px(4.)),
                    ..default()
                }),
            );
        }
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
    // just for testing - to be taken out after proper triggers
) {
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
            &mut AnimationTimer,
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
    sign_query: Query<&Text>,
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
                        player_transform.translation.x = destination.x;
                        player_transform.translation.y = destination.y;
                        *level = LevelSelection::Index(destination.level);
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
