use amethyst::{
  core::{Transform},
  ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{
  CameraTag,
  PlayerTag
};

pub struct CameraFollowSystem;

impl<'s> System<'s> for CameraFollowSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, CameraTag>,
    ReadStorage<'s, PlayerTag>,
  );

  fn run(&mut self, (mut transforms, camera_tags, player_tags): Self::SystemData) {
    let mut player_x = 0.0;
    let mut player_y = 0.0;

    // Get the local position of the player.
    for (_player, transform) in (&player_tags, &transforms).join() {
      player_x = transform.translation().x as f32;
      player_y = transform.translation().y as f32;
    }

    // Update the camera position
    for (_camera, transform) in (&camera_tags, &mut transforms).join() {
      transform.set_translation_x(player_x);
      transform.set_translation_y(player_y);
    }
  }
}
