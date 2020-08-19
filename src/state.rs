use amethyst::{
  assets::{AssetStorage, Loader, Handle},
  prelude::*,
  renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::entities::{
  initialize_background,
  initialize_camera,
  initialize_player
};

#[derive(Default)]
pub struct MyState {
  sprite_sheet_handle: Option<Handle<SpriteSheet>>,
  map_texture_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for MyState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    // load the spritesheets necessary to render the graphics
    self.map_texture_handle.replace(load_map_texture(world));
    self.sprite_sheet_handle.replace(load_sprite_sheet(world));

    initialize_background(world, self.map_texture_handle.clone().unwrap());
    initialize_player(world, self.sprite_sheet_handle.clone().unwrap());
    initialize_camera(world);
  }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
  // Load the sprite sheet necessary to render the graphics.
  // The texture is the pixel data.
  // `texture_handle` is a cloneable reference to the texture
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      "texture/sprite_sheet.png",
      ImageFormat::default(),
      (),
      &texture_storage,
    )
  };

  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    "texture/sprite_sheet.ron", // Here we load the associated ron file
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store,
  )
}

fn load_map_texture(world: &mut World) -> Handle<SpriteSheet> {
  // Load the sprite sheet necessary to render the graphics.
  // The texture is the pixel data.
  // `texture_handle` is a cloneable reference to the texture
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      "texture/background.png",
      ImageFormat::default(),
      (),
      &texture_storage,
    )
  };

  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    "texture/background.ron",
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store,
  )
}
