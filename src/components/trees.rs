use crate::*;

#[derive(Component, Default)]
pub struct TreeBig;

impl TreeBig {
    pub fn spawn(mut commands: Commands, query: Query<Entity, Added<TreeBig>>) {
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
}

#[derive(Bundle, LdtkEntity)]
pub struct TreeBigBundle {
    flag: TreeBig,
    #[from_entity_instance]
    z_from_y: DeriveZFromY,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Component, Default)]
pub struct TreeSmall;

impl TreeSmall {
    pub fn spawn(mut commands: Commands, query: Query<Entity, Added<TreeSmall>>) {
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
}

#[derive(Bundle, LdtkEntity)]
pub struct TreeSmallBundle {
    flag: TreeSmall,
    #[from_entity_instance]
    z_from_y: DeriveZFromY,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}
