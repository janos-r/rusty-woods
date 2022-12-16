use super::*;
use crate::*;

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    sprite_sheet_bundle: SpriteSheetBundle,
    velocity: Velocity,
    #[worldly]
    worldly: Worldly,
    teleporting_to_entity_iid: EntityIid,
}

pub fn spawn_player(
    mut commands: Commands,
    query_player: Query<Entity, Added<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if let Ok(player) = query_player.get_single() {
        let texture_handle = asset_server.load("LDtk/gabe-idle-run.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(24., 24.), 7, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands
            .entity(player)
            .insert(AnimationTimer(Timer::from_seconds(
                0.08,
                TimerMode::Repeating,
            )))
            .insert(texture_atlas_handle)
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::ROTATION_LOCKED)
            // Position the collider relative to the rigid-body.
            .with_children(|parent| {
                parent.spawn((
                    TransformBundle::from(Transform::from_xyz(0., -8., 0.)),
                    Collider::ball(8.),
                ));
            });
    }
}
