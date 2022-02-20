use crate::polar::PolarPoint2;
use nannou::prelude::{deg_to_rad, Point2};
use rand::prelude::Rng;
use std::f32::consts::PI;

pub struct Spinner {
  config: SpinnerConfig,
  state: SpinnerState,
}

impl Spinner {
  pub fn new(config: SpinnerConfig) -> Spinner {
    let mut rng = rand::thread_rng();
    Spinner {
      config,
      state: SpinnerState {
        theta_0: deg_to_rad(360. * rng.gen::<f32>()),
        theta_1: deg_to_rad(360. * rng.gen::<f32>()),
        theta_offset: 0.,
        points: Vec::new(),
      },
    }
  }

  pub fn update(&mut self) {
    self.state.theta_0 = (self.state.theta_0 + self.config.theta_incr) % (2. * PI);
    self.state.theta_1 = (self.state.theta_1 + self.config.theta_incr) % (2. * PI);
    self.state.theta_offset = (self.state.theta_offset + self.config.theta_incr) % (2. * PI);

    let spline = splines::Spline::from_vec(vec![
      splines::Key::new(
        0.,
        Point2::from_polar(0., 0.),
        splines::Interpolation::Bezier(Point2::from_polar(1., self.state.theta_0)),
      ),
      // splines::Key::new(
      //     1.,
      //     Point2::from_polar(1., deg_to_rad(160.)),
      //     splines::Interpolation::Bezier(
      //         2. * Point2::from_polar(1., deg_to_rad(160.))
      //             - Point2::from_polar(1., deg_to_rad(100.)),
      //     ), // point symmetric -> 2*point-original
      splines::Key::new(
        1.,
        Point2::from_polar(1., self.state.theta_1),
        splines::Interpolation::Linear,
      ),
    ]);

    let n_points = (self.config.n_max_points as f32 * self.get_density()) as i32;
    let points = (0..n_points).map(|i| {
      let t = i as f32 / n_points as f32;
      spline.sample(t).unwrap()
    });

    self.state.points.extend(points);
  }

  pub fn get_points(&self) -> &Vec<Point2> {
    &self.state.points
  }

  fn get_density(&self) -> f32 {
    self.config.density_max
      * (1. - self.state.theta_offset * self.config.density_multiplier / (2. * PI))
  }
}

struct SpinnerState {
  theta_0: f32,
  theta_1: f32,
  theta_offset: f32,
  points: Vec<Point2>,
}

pub struct SpinnerConfig {
  theta_incr: f32,
  density_max: f32,
  density_multiplier: f32,
  n_max_points: i32,
}

impl SpinnerConfig {
  pub fn new(
    theta_incr: f32,
    density_max: f32,
    density_multiplier: f32,
    n_max_points: i32,
  ) -> SpinnerConfig {
    SpinnerConfig {
      theta_incr,
      density_max,
      density_multiplier,
      n_max_points,
    }
  }
}
