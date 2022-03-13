mod config;

use crate::polar::PolarPoint2;
use crate::settings::SpinnerSettings;
use crate::spinner::config::SpinnerConfig;
use derive_getters::Getters;
use nannou::prelude::{deg_to_rad, Point2};
use rand::prelude::Rng;
use std::f32::consts::PI;

#[derive(Getters)]
pub struct SpinnerState {
  theta_0: f32,
  theta_1: f32,
  theta_offset: f32,
  points: Vec<Point2>,
}

#[derive(Getters)]
pub struct Spinner {
  config: SpinnerConfig,
  state: SpinnerState,
}

impl Spinner {
  pub fn new(config: SpinnerConfig) -> Self {
    let mut rng = rand::thread_rng();
    Self {
      config,
      state: SpinnerState {
        theta_0: deg_to_rad(360. * rng.gen::<f32>()),
        theta_1: deg_to_rad(360. * rng.gen::<f32>()),
        theta_offset: 0.,
        points: Vec::new(),
      },
    }
  }

  fn step(&mut self) {
    self.state.theta_0 = (self.state.theta_0 + self.config.theta_increment()) % (2. * PI);
    self.state.theta_1 = (self.state.theta_1 + self.config.theta_increment()) % (2. * PI);
    self.state.theta_offset = (self.state.theta_offset + self.config.theta_increment()) % (2. * PI);

    let spline = splines::Spline::from_vec(vec![
      splines::Key::new(
        0.,
        Point2::from_polar(0., 0.),
        splines::Interpolation::Bezier(Point2::from_polar(1., self.state.theta_0)),
      ),
      splines::Key::new(
        1.,
        Point2::from_polar(1., self.state.theta_1),
        splines::Interpolation::Bezier(
          2. * Point2::from_polar(1., self.state.theta_1)
            - Point2::from_polar(1., deg_to_rad(self.state.theta_0)),
        ),
      ),
    ]);

    let n_points = (*self.config.initial_points() as f32 * self.get_density()) as i32;
    let points = (0..n_points).map(|i| {
      let t = i as f32 / n_points as f32;
      spline.sample(t).unwrap() * *self.config.radius() + *self.config.center()
    });

    self.state.points.extend(points);
  }

  pub fn update(&mut self) {
    let iterations = (self.config.theta_max() / self.config.theta_increment()) as usize;
    for _i in 0..iterations {
      self.step();
    }
  }

  fn get_density(&self) -> f32 {
    self.config.density()
      * (1. - self.config().density_factor() * self.state.theta_offset / (2. * PI))
  }
}

impl From<&SpinnerSettings> for Spinner {
  fn from(settings: &SpinnerSettings) -> Self {
    Self::new(SpinnerConfig::from(settings))
  }
}
