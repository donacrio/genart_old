use crate::config::{SpinnerConfig, SpinnerDefaultConfig};
use nannou::prelude::{deg_to_rad, Point2};
use rand::prelude::Rng;
use std::f32::consts::PI;

pub struct Spinner {
  center: Point2,
  density: f32,
  density_factor: f32,
  initial_points: i32,
  radius: f32,
  theta_increment: f32,
  theta_max: f32,
}

pub struct SpinnerInput<'a> {
  config: &'a SpinnerConfig,
  default_config: &'a SpinnerDefaultConfig,
}

impl<'a> SpinnerInput<'a> {
  pub fn new(config: &'a SpinnerConfig, default_config: &'a SpinnerDefaultConfig) -> Self {
    Self {
      config,
      default_config,
    }
  }
}

impl<'a> From<SpinnerInput<'a>> for Spinner {
  fn from(input: SpinnerInput<'a>) -> Self {
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
      theta_increment: input
        .config
        .theta_increment
        .unwrap_or(input.default_config.theta_increment),
      theta_max: input
        .config
        .theta_max
        .unwrap_or(input.default_config.theta_max),
    }
  }
}

impl Spinner {
  pub fn compute_points(&self) -> Vec<Point2> {
    let mut rng = rand::thread_rng();
    let mut theta_0 = deg_to_rad(360. * rng.gen::<f32>());
    let mut theta_1 = deg_to_rad(360. * rng.gen::<f32>());
    let mut theta_offset = 0.;
    let mut points: Vec<Point2> = Vec::new();

    let iterations = (self.theta_max / self.theta_increment) as usize;
    for _i in 0..iterations {
      theta_0 = (theta_0 + self.theta_increment) % (2. * PI);
      theta_1 = (theta_1 + self.theta_increment) % (2. * PI);
      theta_offset = (theta_offset + self.theta_increment) % (2. * PI);

      let spline_start = splines::Key::new(
        0.,
        Point2::from_polar(0., 0.),
        splines::Interpolation::Bezier(Point2::from_polar(1., theta_0)),
      );
      let spline_end = splines::Key::new(
        1.,
        Point2::from_polar(1., theta_1),
        splines::Interpolation::Bezier(
          2. * Point2::from_polar(1., theta_1) - Point2::from_polar(1., deg_to_rad(theta_0)),
        ),
      );
      let spline = splines::Spline::from_vec(vec![spline_start, spline_end]);

      let n_points = (self.initial_points as f32 * self.get_density(theta_offset)) as i32;
      let new_points = (0..n_points).map(|i| {
        let t = i as f32 / n_points as f32;
        spline.sample(t).unwrap() * self.radius + self.center
      });
      points.extend(new_points);
    }

    points
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
