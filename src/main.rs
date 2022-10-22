use bevy::{prelude::*, render::texture::ImageSettings};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy - My testing app ^_^".into(),
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(CurrentWorld(InWorld::W1Main))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(move_camera)
        .add_system(update_transform_from_velocity)
        .add_system(animate_sprite_system_velocity)
        .add_system(switch_world)
        .run();
}

#[derive(Component, Default)]
struct Player;

#[derive(Component, Default, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Default)]
struct Velocity(Vec3);

// For switching between visible worlds
#[derive(Component, PartialEq)]
enum InWorld {
    W1Main,
    W2,
}
struct CurrentWorld(InWorld);

// Text box
#[derive(Component)]
struct TextBoxContainer;
#[derive(Component)]
struct TextBox;

// Local Bundles
#[derive(Bundle, Default)]
struct PlayerBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    animation_timer: AnimationTimer,
    player: Player,
    velocity: Velocity,
}

#[derive(Bundle)]
struct MySpriteBundle {
    in_world: InWorld,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Player
    commands.spawn_bundle(PlayerBundle {
        animation_timer: AnimationTimer(Timer::from_seconds(0.08, true)),
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        ..default()
    });

    // Sprites
    commands.spawn_bundle(MySpriteBundle {
        in_world: InWorld::W1Main,
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                custom_size: Some(Vec2::new(200.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -100., 0.)),
            ..default()
        },
    });

    commands.spawn_bundle(MySpriteBundle {
        in_world: InWorld::W2,
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.1),
                custom_size: Some(Vec2::new(50.0, 200.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(200., 0., 0.)),
            visibility: Visibility { is_visible: false },
            ..default()
        },
    });

    // Bottom text box
    let text_box_width = 1000.;
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(200.0)),
                // makes space bellow the box
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                ..default()
            },
            // transparent container
            color: Color::rgba(0.65, 0.65, 0.65, 0.).into(),
            ..default()
        })
        .insert(TextBoxContainer)
        .with_children(|parent| {
            // box size, border thickness and color
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        // because for now, text can't wrap in width by percentage
                        size: Size::new(Val::Px(text_box_width), Val::Percent(90.0)),
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    color: Color::MIDNIGHT_BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // text background
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent
                                .spawn_bundle(
                                    TextBundle::from_section(
                                        "Text Example",
                                        // "Text Example a little longer trying to cross the width. Text Example a little longer trying to cross the width",
                                        // "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularized in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_style(Style {
                                        margin: UiRect::all(Val::Px(10.0)),
                                        // For now broken, commented myself on: https://github.com/bevyengine/bevy/issues/5834
                                        // size: Size {
                                        //     // `Val::Percent` doesn't work currently for wrapping
                                        //     width: Val::Px(600.),
                                        //     ..default()
                                        // },
                                        ..default()
                                    }),
                                )
                                .insert(TextBox);
                        });
                });
        });
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
    // just for testing - to be taken out after proper triggers
    mut current_world: ResMut<CurrentWorld>,
    mut text_box_visibility: Query<&mut Visibility, With<TextBoxContainer>>,
    mut text_box: Query<&mut Text, With<TextBox>>,
) {
    let mut player_velocity = query.single_mut();
    const SPEED: f32 = 5.;

    let default = Vec3::default();
    if player_velocity.0 != default {
        player_velocity.0 = default;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        player_velocity.0 += Vec3::new(-SPEED, 0., 0.);

        // TODO:
        // after collision detection, create doors to change the current world
        // enter W1
        current_world.0 = InWorld::W1Main;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        player_velocity.0 += Vec3::new(SPEED, 0., 0.);

        // enter W2
        current_world.0 = InWorld::W2;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        player_velocity.0 += Vec3::new(0., SPEED, 0.);

        // TODO:
        // create proper trigger for the text box
        text_box_visibility.single_mut().is_visible = true;
        text_box.single_mut().sections[0].value =
            "A totally new text from the trigger ^^".to_owned()
    }

    if keyboard_input.pressed(KeyCode::Down) {
        player_velocity.0 += Vec3::new(0., -SPEED, 0.);
        text_box_visibility.single_mut().is_visible = false;
    }
}

fn move_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, With<Camera>), Without<Player>>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut().0;
    camera_transform.translation = player_transform.translation;
}

fn update_transform_from_velocity(
    mut query: Query<(&mut Transform, &Velocity), Changed<Velocity>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0;
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
        if velocity.0 == Vec3::default() {
            sprite.index = 0;
        } else if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn switch_world(current_world: Res<CurrentWorld>, mut query: Query<(&mut Visibility, &InWorld)>) {
    if current_world.is_changed() {
        for (mut visibility, in_world) in &mut query {
            visibility.is_visible = in_world == &current_world.0
        }
    }
}
