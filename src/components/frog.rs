use crate::*;

#[derive(Component, Default)]
pub struct Frog;

#[derive(Bundle, LdtkEntity)]
pub struct FrogBundle {
    frog: Frog,
    z_from_y: DeriveZFromY,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    sprite_sheet_animation: SpriteSheetAnimation,
    #[with(collider)]
    collider: Collider,
}

fn collider(_: EntityInstance) -> Collider {
    Collider::ball(12.)
}
