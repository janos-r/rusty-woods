use crate::*;

#[allow(clippy::too_many_arguments)]
pub fn collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    // ↓ Doors
    mut player_target_query: Query<&mut EntityIid, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    door_destination_query: Query<&DoorRef>,
    // ↓ Signs
    sign_collider_query: Query<&Parent, With<SignCollider>>,
    sign_query: Query<&SignText>,
    text_box_query: Query<(Entity, &Children, &Handle<Font>), With<TextBox>>,
    mut commands: Commands,
    mut text_box_visibility: Query<&mut Visibility, With<TextBoxContainer>>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            // lets not hope that the door will always be e1 - lets try both and also stop if it was first
            for entity in [e1, e2] {
                if let Ok(destination) = door_destination_query.get(*entity) {
                    // Door - switch_level
                    if let Ok(mut player_target_iid) = player_target_query.get_single_mut() {
                        let DoorRef {
                            target_entity_iid,
                            target_level_iid,
                        } = destination.clone();
                        *player_target_iid = target_entity_iid;
                        *level_selection = target_level_iid;
                    }
                    break;
                } else if let Ok(parent) = sign_collider_query.get(*entity) {
                    // Sign - display text
                    if let Ok(text) = sign_query.get(parent.get()) {
                        // clear text
                        // despawning children (here the words)
                        // issue: https://bevy-cheatbook.github.io/features/parent-child.html?highlight=remove_chil#despawning-child-entities
                        let (entity, children, font_handle) = text_box_query.single();
                        commands.entity(entity).remove_children(children);
                        for child in children {
                            commands.entity(*child).despawn_recursive();
                        }
                        // open text_box
                        text_box_visibility.single_mut().is_visible = true;
                        // new text
                        commands.entity(entity).add_children(spawn_children_text(
                            font_handle.clone(),
                            text.0.to_owned(),
                        ));
                        break;
                    }
                };
            }
        } else if let CollisionEvent::Stopped(e1, e2, _) = collision_event {
            for entity in [e1, e2] {
                if sign_collider_query.contains(*entity) {
                    // close text_box
                    text_box_visibility.single_mut().is_visible = false;
                    break;
                }
            }
        }
    }
}
