use crate::*;

#[derive(Component, Default)]
pub struct Sign;

#[derive(Component, Default)]
// separate entity - so that it can be located relative to the sign
pub struct SignCollider;

#[derive(Component, Default)]
pub struct SignText(pub String);

#[derive(Bundle, LdtkEntity)]
pub struct SignBundle {
    sign: Sign,
    z_from_y: DeriveZFromY,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    text: SignText,
}

impl From<EntityInstance> for SignText {
    fn from(entity_instance: EntityInstance) -> SignText {
        let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "String") else {
            return default();
        };
        let FieldValue::String(Some(text)) = &field_instance.value else {
            return default();
        };
        SignText(text.to_owned())
    }
}

pub fn spawn_sign(mut commands: Commands, query_sign: Query<Entity, Added<Sign>>) {
    for sign in &query_sign {
        commands.entity(sign).with_children(|parent| {
            parent.spawn((
                SignCollider,
                // Position the collider relative to the rigid-body.
                TransformBundle::from(Transform::from_xyz(0., -2., 0.)),
                Collider::ball(6.),
                ActiveEvents::COLLISION_EVENTS,
            ));
        });
    }
}
