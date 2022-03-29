use nannou::prelude::{Draw, Srgba, Vec2};
use rand::prelude::Rng;
use std::f32::consts::PI;

pub struct Traveler {
  pub position: Vec2,
  pub velocity: Vec2,
  acceleration: Vec2,
  n_points: usize,
  max_velocity: f32,
  max_force: f32,
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

  fn apply_force(&mut self, force: Vec2) {
    self.acceleration = self.acceleration + force;
  }

  pub fn seek(&mut self, position: Vec2) {
    let desired_velocity = position - self.position;
    let desired_velocity = desired_velocity.clamp_length_max(self.max_velocity);
    let force = desired_velocity - self.velocity;
    let force = force.clamp_length_max(self.max_force);
    self.apply_force(force);
  }

  pub fn avoid_border(&mut self, width: f32, height: f32, radius: f32) {
    if (self.position.x - width / 2.).abs() < radius {
      let position = Vec2::new(0., self.position.y);
      self.seek(position);
    }
    if (self.position.y - height / 2.).abs() < radius {
      let position = Vec2::new(self.position.x, 0.);
      self.seek(position);
    }
  }

  pub fn draw(&self, draw: &Draw, target: &Vec2, color: &Srgba) {
    let mut rng = rand::thread_rng();

    let middle = (self.position + *target) / 2.;
    let direction = *target - self.position;

    for _ in 0..self.n_points {
      let theta = 2. * PI * rng.gen::<f32>();
      let point = middle + theta.sin() / 2. * direction;
      draw.ellipse().xy(point).w_h(1., 1.).color(*color);
      let point = middle - theta.sin() / 2. * direction;
      draw.ellipse().xy(point).w_h(1., 1.).color(*color);
    }
  }
}
