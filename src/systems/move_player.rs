use amethyst::{
  core::{Transform},
  ecs::{Join, Read, System, WriteStorage},
  input::{InputHandler, StringBindings},
};

use crate::components::{
  Action,
  ActionId,
};

pub struct MovePlayerSystem;

impl <'s> System<'s> for MovePlayerSystem {
  type SystemData = (
    WriteStorage<'s, Action>,
    WriteStorage<'s, Transform>,
    Read<'s, InputHandler<StringBindings>>,
  );

  fn run(&mut self, (mut actions,mut transforms, input): Self::SystemData) {
    for (action, transform) in (&mut actions, &mut transforms).join() {
      let movement_x: f32 = input.axis_value("player_move_x").unwrap();
      let movement_y: f32 = input.axis_value("player_move_y").unwrap();

      if movement_x != 0.0 {
        let scaled_amount = 0.65 * movement_x;
        let x_pos = transform.translation().x;
        // Direction the player is facing. If west, then rotate 180.
        let rotation = if movement_x < 0.0 { 3.14 } else { 0.0 };

        action.set_action(ActionId::Run);
        transform.set_rotation_y_axis(rotation);
        transform.set_translation_x(x_pos + scaled_amount);
      } else if movement_y != 0.0 {
        let scaled_amount = 0.65 * movement_y;
        let y_pos = transform.translation().y;

        action.set_action(ActionId::Run);
        transform.set_translation_y(y_pos + scaled_amount);
      } else {
        action.set_action(ActionId::Idle);
      }
    }
  }
}
