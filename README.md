### TODOs

- z-axis
  - having it derived from the y-axis could make it possible to walk behind objects
- walls
  - wall collider combiner
    - compare platformer vs ludum and use one
- add new free sprite sheet
  - source: <https://itch.io/game-assets/free/tag-2d/tag-sprites>
  - use doors on sprites
  - combine with walls

### Optional

- interaction with something in front
- save_game
    found a good example at:
    <https://github.com/bevyengine/bevy/issues/1442>

bevy game list\
<https://itch.io/search?q=bevy>

### Issues I found workarounds for

To look for real fixes in the future, if someone is looking to build upon this code

- UI wrapping text
  - Bevy issue - see in `ui.rs` under "Known issue" <https://github.com/bevyengine/bevy/issues/1490>

- Accessing entity_refs outside the current level
  - bevy_ecs_ldtk issue - see in `door.rs` under "Known issue"
  <https://github.com/Trouv/bevy_ecs_ldtk/discussions/113>
