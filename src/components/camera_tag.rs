use amethyst::{ecs::prelude::{Component, NullStorage}};

#[derive(Default)]
pub struct CameraTag;

impl Component for CameraTag {
  type Storage = NullStorage<Self>;
}
