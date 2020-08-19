use amethyst::{
  assets::{Handle},
  core::transform::Transform,
  prelude::*,
  renderer::{SpriteRender, SpriteSheet},
};

pub fn initialize_background(world: &mut World, map_texture_handle: Handle<SpriteSheet>) {
  // Create the translation
  let mut local_transform = Transform::default();
  local_transform.set_translation_xyz(0.0, 0.0, -10.0);

  let map_render = SpriteRender {
    sprite_sheet: map_texture_handle,
    sprite_number: 0,
  };

  world
    .create_entity()
    .with(map_render)
    .with(local_transform)
    .build();
}
