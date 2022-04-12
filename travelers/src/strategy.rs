use crate::traveler::Traveler;
use nannou::prelude::Vec2;

pub enum Strategy {
  SEEK,
}

pub fn apply_strategy(
  traveler: &mut Traveler,
  position: &Vec2,
  strategy: Strategy,
  max_force: f32,
) {
  match strategy {
    Strategy::SEEK => seek(traveler, position, max_force),
  }
}

fn seek(traveler: &mut Traveler, position: &Vec2, max_force: f32) {
  let desired_velocity = *position - traveler.position;
  let desired_velocity = desired_velocity.clamp_length_max(traveler.max_velocity);
  let force = desired_velocity - traveler.velocity;
  let force = force.clamp_length_max(max_force);
  apply_force(traveler, force);
}

fn apply_force(traveler: &mut Traveler, force: Vec2) {
  traveler.acceleration += force;
}
