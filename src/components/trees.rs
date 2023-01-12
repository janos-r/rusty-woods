use crate::*;

#[derive(Component, Default)]
pub struct TreeBig;

#[derive(Bundle, LdtkEntity)]
pub struct TreeBigBundle {
    flag: TreeBig,
    #[with(super::derive_z_from_y)]
    z_from_y: DeriveZFromY,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

pub fn spawn_tree_big(mut commands: Commands, query: Query<Entity, Added<TreeBig>>) {
    for entity in &query {
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                SignCollider,
                // Position the collider relative to the rigid-body.
                TransformBundle::from(Transform::from_xyz(0., -17., 0.)),
                Collider::ball(11.),
            ));
        });
    }
}

#[derive(Component, Default)]
pub struct TreeSmall;

#[derive(Bundle, LdtkEntity)]
pub struct TreeSmallBundle {
    flag: TreeSmall,
    #[with(super::derive_z_from_y)]
    z_from_y: DeriveZFromY,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

pub fn spawn_tree_small(mut commands: Commands, query: Query<Entity, Added<TreeSmall>>) {
    for entity in &query {
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                // Position the collider relative to the rigid-body.
                TransformBundle::from(Transform::from_xyz(0., -12., 0.)),
                Collider::ball(6.),
            ));
        });
    }
}
