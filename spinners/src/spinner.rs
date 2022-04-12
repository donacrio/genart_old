use crate::config::{SpinnerConfig, SpinnerDefaultConfig};
use nannou::prelude::{deg_to_rad, Point2, Srgb};
use rand::prelude::Rng;
use std::f32::consts::PI;

pub struct SpinnerDrawOptions {
  pub color: Srgb<u8>,
  pub point_weight: f32,
}

pub struct Spinner {
  center: Point2,
  density: f32,
  density_factor: f32,
  initial_points: i32,
  radius: f32,
  theta_increment: f32,
  theta_0: f32,
  theta_1: f32,
  theta_offset: f32,
  max_iterations: usize,
  pub draw_options: SpinnerDrawOptions,
}

pub struct SpinnerInput<'a> {
  config: &'a SpinnerConfig,
  default_config: &'a SpinnerDefaultConfig,
  rng: &'a mut rand::rngs::SmallRng,
}

impl<'a> SpinnerInput<'a> {
  pub fn new(
    config: &'a SpinnerConfig,
    default_config: &'a SpinnerDefaultConfig,
    rng: &'a mut rand::rngs::SmallRng,
  ) -> Self {
    Self {
      config,
      default_config,
      rng,
    }
  }
}

impl<'a> From<SpinnerInput<'a>> for Spinner {
  fn from(input: SpinnerInput<'a>) -> Self {
    let theta_increment = input
      .config
      .theta_increment
      .unwrap_or(input.default_config.theta_increment);
    let max_iterations = input
      .config
      .theta_max
      .unwrap_or(input.default_config.theta_max)
      / theta_increment;
    Self {
      center: input.config.center,
      density: input.config.density.unwrap_or(input.default_config.density),
      density_factor: input
        .config
        .density_factor
        .unwrap_or(input.default_config.density_factor),
      initial_points: input
        .config
        .initial_points
        .unwrap_or(input.default_config.initial_points),
      radius: input.config.radius.unwrap_or(input.default_config.radius),
      theta_increment,
      theta_0: deg_to_rad(360. * input.rng.gen::<f32>()),
      theta_1: deg_to_rad(360. * input.rng.gen::<f32>()),
      theta_offset: 0.,
      max_iterations: max_iterations as usize,
      draw_options: SpinnerDrawOptions {
        color: input
          .config
          .drawing
          .as_ref()
          .map(|drawing| drawing.color)
          .unwrap_or(input.default_config.drawing.color),
        point_weight: input
          .config
          .drawing
          .as_ref()
          .map(|drawing| drawing.point_weight)
          .unwrap_or(input.default_config.drawing.point_weight),
      },
    }
  }
}

impl Spinner {
  pub fn compute_points(&mut self, iteration: usize) -> Vec<Point2> {
    if iteration <= self.max_iterations {
      self.theta_0 = (self.theta_0 + self.theta_increment) % (2. * PI);
      self.theta_1 = (self.theta_1 + self.theta_increment) % (2. * PI);
      self.theta_offset = (self.theta_offset + self.theta_increment) % (2. * PI);

      let spline_start = splines::Key::new(
        0.,
        Point2::from_polar(0., 0.),
        splines::Interpolation::Bezier(Point2::from_polar(1., self.theta_0)),
      );
      let spline_end = splines::Key::new(
        1.,
        Point2::from_polar(1., self.theta_1),
        splines::Interpolation::Bezier(
          2. * Point2::from_polar(1., self.theta_1)
            - Point2::from_polar(1., deg_to_rad(self.theta_0)),
        ),
      );
      let spline = splines::Spline::from_vec(vec![spline_start, spline_end]);

      let n_points = (self.initial_points as f32 * self.get_density(self.theta_offset)) as i32;
      let points = (0..n_points)
        .map(|i| {
          let t = i as f32 / n_points as f32;
          spline.sample(t).unwrap() * self.radius + self.center
        })
        .collect();
      return points;
    }
    return Vec::new();
  }

  fn get_density(&self, offset: f32) -> f32 {
    self.density * (1. - self.density_factor * offset / (2. * PI))
  }
}

pub trait PolarPoint2 {
  fn from_polar(radius: f32, angle: f32) -> Self;
}

impl PolarPoint2 for Point2 {
  fn from_polar(radius: f32, radian: f32) -> Self {
    Self::new(radius * radian.cos(), radius * radian.sin())
  }
}
