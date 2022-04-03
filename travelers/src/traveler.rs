use nannou::prelude::Vec2;

pub struct Traveler {
  pub position: Vec2,
  pub velocity: Vec2,
  pub acceleration: Vec2,
  pub max_velocity: f32,
  pub max_force: f32,
  pub n_points: usize,
}

impl Traveler {
  pub fn new(
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    n_points: usize,
    max_velocity: f32,
    max_force: f32,
  ) -> Self {
    Self {
      position,
      velocity,
      acceleration,
      n_points,
      max_velocity,
      max_force,
    }
  }

  pub fn update(&mut self) {
    self.velocity += self.acceleration;
    self.velocity = self.velocity.clamp_length_max(self.max_velocity);
    self.position += self.velocity;
    self.acceleration = Vec2::splat(0.);
  }
}
