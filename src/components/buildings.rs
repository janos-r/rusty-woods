use crate::*;

#[derive(Component, Default)]
pub struct ToriiGate;
impl ToriiGate {
    const PILLAR_SIZE: f32 = 3.;
}

#[derive(Bundle, LdtkEntity)]
pub struct ToriiGateBundle {
    torii_gate: ToriiGate,
    #[with(super::derive_z_from_y)]
    z_from_y: DeriveZFromY,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

pub fn spawn_torii_gate(mut commands: Commands, query: Query<Entity, Added<ToriiGate>>) {
    for entity in &query {
        commands.entity(entity).with_children(|parent| {
            // Spawn two colliders (pillars) relative to the rigid-body.
            parent.spawn((
                TransformBundle::from(Transform::from_xyz(-16., -16., 0.)),
                Collider::ball(ToriiGate::PILLAR_SIZE),
            ));
            parent.spawn((
                TransformBundle::from(Transform::from_xyz(16., -16., 0.)),
                Collider::ball(ToriiGate::PILLAR_SIZE),
            ));
        });
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct HouseBundle {
    #[with(super::derive_z_from_y)]
    z_from_y: DeriveZFromY,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}
