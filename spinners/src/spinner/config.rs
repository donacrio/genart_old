use crate::settings::{SpinnerDrawingSettings, SpinnerSettings};
use derive_getters::Getters;
use nannou::prelude::{Point2, Srgb};

#[derive(Getters)]
pub struct SpinnerDrawingConfig {
  color: Srgb<u8>,
  point_weight: f32,
}

impl SpinnerDrawingConfig {
  pub fn new(color: Srgb<u8>, point_weight: f32) -> Self {
    Self {
      color,
      point_weight,
    }
  }
}

impl From<&SpinnerDrawingSettings> for SpinnerDrawingConfig {
  fn from(settings: &SpinnerDrawingSettings) -> Self {
    Self::new(settings.color.into_format(), settings.point_weight)
  }
}

#[derive(Getters)]
pub struct SpinnerConfig {
  center: Point2,
  density: f32,
  density_factor: f32,
  drawing: SpinnerDrawingConfig,
  initial_points: i32,
  radius: f32,
  theta_increment: f32,
  theta_max: f32,
}

impl SpinnerConfig {
  pub fn new(
    center: Point2,
    density: f32,
    density_factor: f32,
    drawing: SpinnerDrawingConfig,
    initial_points: i32,
    radius: f32,
    theta_increment: f32,
    theta_max: f32,
  ) -> Self {
    Self {
      center,
      density,
      density_factor,
      drawing,
      initial_points,
      radius,
      theta_increment,
      theta_max,
    }
  }
}

impl From<&SpinnerSettings> for SpinnerConfig {
  fn from(settings: &SpinnerSettings) -> Self {
    Self::new(
      settings.center,
      settings.density,
      settings.density_factor,
      SpinnerDrawingConfig::from(&settings.drawing),
      settings.initial_points,
      settings.radius,
      settings.theta_increment,
      settings.theta_max,
    )
  }
}
