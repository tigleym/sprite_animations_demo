use amethyst::{ecs::prelude::{Component, DenseVecStorage}};

#[derive(PartialEq, Eq)]
pub enum ActionId {
  Run,
  Idle,
}

pub struct Action {
  pub action_id: ActionId,
}

impl Action {
  pub fn set_action(&mut self, action: ActionId) {
    self.action_id = action;
  }
}

impl Component for Action {
  type Storage = DenseVecStorage<Self>;
}
