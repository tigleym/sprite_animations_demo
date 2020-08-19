use amethyst::{
  assets::{Handle},
  core::transform::Transform,
  ecs::{Entity},
  prelude::*,
  renderer::{SpriteRender, SpriteSheet},
};

use crate::components::{
  Animation,
  Action,
  ActionId,
  PlayerTag,
};

pub fn initialize_player(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) -> Entity {
  // Create the translation
  let mut local_transform = Transform::default();
  local_transform.set_translation_xyz(0.0, 0.0, 0.0);

  let animation = Animation {
    frames: 4,
    frame_duration: 10,
    first_sprite_index: 1, // the first frame for this example is the first sprite.
  };
  let action = Action {
    action_id: ActionId::Idle,
  };
  let sprite_render = SpriteRender {
    sprite_sheet: sprite_sheet_handle,
    sprite_number: animation.first_sprite_index,
  };
  let player_tag = PlayerTag::default();

  world
    .create_entity()
    .with(sprite_render)
    .with(animation)
    .with(action)
    .with(local_transform)
    .with(player_tag)
    .build()
}
