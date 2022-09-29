use bevy::{prelude::*, render::texture::ImageSettings};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy - My testing app ^_^".into(),
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(update_transform_from_velocity)
        .add_system(animate_sprite_system_velocity)
        .run();
}

#[derive(Component)]
struct MovePlayer;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Default)]
struct Velocity(Vec3);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MovePlayer)
        .insert(Velocity::default());

    // Player
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(MovePlayer)
        .insert(Velocity::default());

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.7, 0.7, 0.7),
            custom_size: Some(Vec2::new(200.0, 50.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., -100., 0.)),
        ..default()
    });
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<MovePlayer>>,
) {
    for mut velocity in &mut query {
        const SPEED: f32 = 5.;

        let default = Vec3::default();
        if velocity.0 != default {
            velocity.0 = default;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            velocity.0 += Vec3::new(-SPEED, 0., 0.);
        }

        if keyboard_input.pressed(KeyCode::Right) {
            velocity.0 += Vec3::new(SPEED, 0., 0.);
        }

        if keyboard_input.pressed(KeyCode::Up) {
            velocity.0 += Vec3::new(0., SPEED, 0.);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            velocity.0 += Vec3::new(0., -SPEED, 0.);
        }
    }
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
            sprite.index = (sprite.index as usize + 1) % texture_atlas.textures.len();
        }
    }
}
