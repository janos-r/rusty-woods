pub mod collision_events;
pub use collision_events::*;

use crate::*;

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Moving), With<Player>>,
) {
    if let Ok((mut velocity, mut moving)) = query.get_single_mut() {
        const SPEED: f32 = 200.;

        let default = Vect::default();
        if velocity.linvel != default {
            velocity.linvel = default;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            velocity.linvel += Vect::new(-SPEED, 0.);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            velocity.linvel += Vect::new(SPEED, 0.);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            velocity.linvel += Vect::new(0., SPEED);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            velocity.linvel += Vect::new(0., -SPEED);
        }

        if velocity.linvel != default {
            moving.0 = true;
        } else if moving.0 {
            moving.0 = false;
        }
    }
}

pub fn player_derive_z_from_y(
    mut player_query: Query<&mut Transform, (Changed<Transform>, With<Player>)>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        transform.translation.z = DeriveZFromY::get(transform.translation.y);
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

pub fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut components::AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &Moving,
        ),
        Changed<Moving>,
    >,
) {
    if let Ok((mut timer, mut sprite, texture_atlas_handle, moving)) = query.get_single_mut() {
        timer.tick(time.delta());
        if !moving.0 {
            sprite.index = 0;
        } else if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
