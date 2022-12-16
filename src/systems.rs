pub mod collision_events;
pub use collision_events::*;

use crate::*;

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
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

pub fn move_derive_z_from_y(
    mut player_query: Query<&mut Transform, (Changed<Transform>, With<Player>)>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        transform.translation.z = DeriveZfromY::get(transform.translation.y);
    }
}

pub fn move_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}

pub fn animate_sprite_system_velocity(
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
