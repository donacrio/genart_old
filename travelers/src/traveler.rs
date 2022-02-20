use nannou::prelude::Vec2;

pub struct Traveler {
  pub position: Vec2,
  pub velocity: Vec2,
  acceleration: Vec2,
  max_velocity: f32,
  max_force: f32,
}

impl Traveler {
  pub fn new(
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    max_velocity: f32,
    max_force: f32,
  ) -> Self {
    Self {
      position,
      velocity,
      acceleration,
      max_velocity,
      max_force,
    }
  }
}

impl Traveler {
  pub fn update(&mut self) {
    self.velocity += self.acceleration;
    self.velocity = self.velocity.clamp_length_max(self.max_velocity);
    self.position += self.velocity;
    self.acceleration = Vec2::splat(0.);
  }

  fn apply_force(&mut self, force: Vec2) {
    self.acceleration = self.acceleration + force;
  }

  pub fn seek(&mut self, target: Vec2) {
    let desired_velocity = target - self.position;
    let desired_velocity = desired_velocity.clamp_length_max(self.max_velocity);
    let force = desired_velocity - self.velocity;
    let force = force.clamp_length_max(self.max_force);
    self.apply_force(force);
  }

  pub fn avoid_border(&mut self, width: f32, height: f32, radius: f32) {
    if (self.position.x - width / 2.).abs() < radius {
      let target = Vec2::new(0., self.position.y);
      self.seek(target);
    }
    if (self.position.y - height / 2.).abs() < radius {
      let target = Vec2::new(self.position.x, 0.);
      self.seek(target);
    }
  }
}
