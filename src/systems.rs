pub mod animation;
pub mod collision_events;
pub use animation::*;
pub use collision_events::*;

use crate::*;

// Systems - in this module, I keep systems that run on every tick.
// Systems for spawning entities and theirs components I keep in components.

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &mut Velocity,
            &mut TextureAtlasSprite,
            &mut PlayerAnimationState,
            &mut SpriteSheetAnimation,
        ),
        With<Player>,
    >,
) {
    if let Ok((mut velocity, mut sprite, mut state, mut animation)) = query.get_single_mut() {
        const SPEED: f32 = 150.;

        let default = Vect::default();
        if velocity.linvel != default {
            velocity.linvel = default;
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            velocity.linvel += Vect::new(0., SPEED);
        }
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            velocity.linvel += Vect::new(-SPEED, 0.);
            sprite.flip_x = true;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            velocity.linvel += Vect::new(0., -SPEED);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            velocity.linvel += Vect::new(SPEED, 0.);
            sprite.flip_x = false;
        }

        if velocity.linvel != default {
            if *state != PlayerAnimationState::Running {
                // start running
                *state = PlayerAnimationState::Running;
                *animation = PlayerAnimationState::Running.into();
            }
        } else if *state != PlayerAnimationState::Idle {
            // stop running
            *state = PlayerAnimationState::Idle;
            *animation = PlayerAnimationState::Idle.into();
        }
    }
}

pub fn derive_z_from_y_after_move(
    mut player_query: Query<(&mut Transform, &DeriveZFromY), Changed<Transform>>,
) {
    if let Ok((mut transform, dzfy)) = player_query.get_single_mut() {
        transform.translation.z = dzfy.get(transform.translation.y);
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
