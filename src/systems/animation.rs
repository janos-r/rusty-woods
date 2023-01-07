use std::ops::Range;

use crate::*;

#[derive(Component)]
pub struct SpriteSheetAnimation {
    pub state_range: Option<Range<usize>>,
    pub timer: Timer,
}

impl Default for SpriteSheetAnimation {
    fn default() -> Self {
        Self {
            state_range: None,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

pub fn animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
        &mut SpriteSheetAnimation,
    )>,
) {
    for (texture_atlas_handle, mut sprite, mut animation) in &mut query {
        if let Some(state_range) = animation.state_range.clone() {
            if animation.is_changed() {
                sprite.index = state_range.start;
            } else {
                animation.timer.tick(time.delta());
                if animation.timer.finished() {
                    let next_index = sprite.index + 1;
                    sprite.index = if next_index < state_range.end {
                        next_index
                    } else {
                        state_range.start
                    }
                }
            }
        } else {
            // for simple sheets (without states)
            // so that you don't have to always setup the exact number of frames if a sheet has just one state
            animation.timer.tick(time.delta());
            if animation.timer.finished() {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
        };
    }
}
