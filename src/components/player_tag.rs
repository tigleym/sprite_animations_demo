use amethyst::{ecs::prelude::{Component, NullStorage}};

#[derive(Default)]
pub struct PlayerTag;

impl Component for PlayerTag {
  type Storage = NullStorage<Self>;
}
