use std::ops::Range;

use crate::*;

#[derive(Default, Component)]
pub struct SpriteSheetAnimation {
    pub indices: Range<usize>,
    pub timer: Timer,
}

pub fn animation(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut SpriteSheetAnimation)>,
) {
    if let Ok((mut sprite, mut animation)) = query.get_single_mut() {
        if animation.is_changed() {
            sprite.index = animation.indices.start;
        } else {
            animation.timer.tick(time.delta());
            if animation.timer.finished() {
                let next_index = sprite.index + 1;
                sprite.index = if next_index < animation.indices.end {
                    next_index
                } else {
                    animation.indices.start
                }
            }
        }
    }
}
