use crate::*;

#[derive(Component, Default)]
pub struct Sign;

impl Sign {
    pub fn spawn(mut commands: Commands, query: Query<Entity, Added<Sign>>) {
        for entity in &query {
            commands.entity(entity).with_children(|parent| {
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
}

// so that it can be located relative to the sign, and have ActiveEvents
#[derive(Component, Default)]
pub struct SignCollider;

#[derive(Component, Default)]
pub struct SignText(pub String);

#[derive(Bundle, LdtkEntity)]
pub struct SignBundle {
    sign: Sign,
    #[from_entity_instance]
    z_from_y: DeriveZFromY,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    text: SignText,
}

impl From<&EntityInstance> for SignText {
    fn from(entity_instance: &EntityInstance) -> SignText {
        let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "String")
        else {
            return default();
        };
        let FieldValue::String(Some(text)) = &field_instance.value else {
            return default();
        };
        SignText(text.clone())
    }
}
