### TODOs

- add new free sprite sheet
  - redo Objects-set as entities
  - add houses
    - interior tiles
- animation enhancements
  - patrol

### Optional

- interaction with something in front
- save_game
    found a good example at:
    <https://github.com/bevyengine/bevy/issues/1442>

bevy game list\
<https://itch.io/search?q=bevy>

### Sprite source

- player gabe - bevy assets
- plains(grass, dirt, higher ground), decor, fences, objects (trees, rocks, signs) <https://game-endeavor.itch.io/mystic-woods>
- frog, houses <https://pixel-boy.itch.io/ninja-adventure-asset-pack>

### Issues I found workarounds for

To look for real fixes in the future, if someone is looking to build upon this code

- UI wrapping text
  - Bevy issue - see in `ui.rs` under "Known issue" <https://github.com/bevyengine/bevy/issues/1490>

- Accessing entity_refs outside the current level
  - bevy_ecs_ldtk issue - see in `door.rs` under "Known issue"
  <https://github.com/Trouv/bevy_ecs_ldtk/discussions/113>

- Sprite selection from multiple tiles loads the wrong tiles
  - bevy_ecs_ldtk issue - not possible to use `torii_gate` see ToriiGateBundle
  <https://github.com/Trouv/bevy_ecs_ldtk/issues/151>

- Rules based tiles don't use pivot
  - bevy_ecs_ldtk issue - some ground objects had to be remade as entities
  <https://github.com/Trouv/bevy_ecs_ldtk/issues/152>
