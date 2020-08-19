use amethyst::{
  core::{Transform},
  prelude::*,
  renderer::{Camera},
};
use crate::components::{CameraTag};

const CAMERA_HEIGHT: f32 = 200.0;
const CAMERA_WIDTH: f32 = 200.0;

pub fn initialize_camera(world: &mut World) {
  let mut transform = Transform::default();
  transform.set_translation_z(5.0);

  let camera_tag = CameraTag::default();

  world
    .create_entity()
    .with(Camera::standard_2d(CAMERA_WIDTH, CAMERA_HEIGHT))
    .with(transform)
    .with(camera_tag)
    .build();
}
