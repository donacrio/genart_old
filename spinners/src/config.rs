use nannou::prelude::{Point2, Srgb};
use serde::Deserialize;
use std::path::Path;

const CONFIG_DEFAULT_PATH: &str = "configs/spinners/default.toml";

pub fn load_config(path: Option<String>) -> Config {
  let path = path.as_ref().map(|path| path.as_str()).unwrap_or_else(|| {
    println!("Could not find configuration file path. Using default configuration");
    CONFIG_DEFAULT_PATH
  });

  match Config::new(path) {
    Ok(config) => config,
    Err(err) => {
      println!("Encountered error: {}.\nProcess exited with code 1", err);
      std::process::exit(1);
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct WindowConfig {
  pub background_color: Srgb<u8>,
  pub height: u32,
  pub width: u32,
}

#[derive(Deserialize)]
pub struct SpinnerDrawingConfig {
  pub color: Srgb<u8>,
  pub point_weight: f32,
}

#[derive(Deserialize)]
pub struct SpinnerConfig {
  pub center: Point2,
  pub density: Option<f32>,
  pub density_factor: Option<f32>,
  pub drawing: Option<SpinnerDrawingConfig>,
  pub initial_points: Option<i32>,
  pub radius: Option<f32>,
  pub theta_increment: Option<f32>,
  pub theta_max: Option<f32>,
}

#[derive(Deserialize)]
pub struct SpinnerDefaultConfig {
  pub density: f32,
  pub density_factor: f32,
  pub drawing: SpinnerDrawingConfig,
  pub initial_points: i32,
  pub radius: f32,
  pub theta_increment: f32,
  pub theta_max: f32,
}

#[derive(Deserialize)]
pub struct Config {
  pub window: WindowConfig,
  pub spinners: Vec<SpinnerConfig>,
  pub spinner_default_config: SpinnerDefaultConfig,
}

impl Config {
  pub fn new(path: &str) -> Result<Self, config::ConfigError> {
    config::Config::builder()
      .add_source(config::File::from(Path::new(path)))
      .build()?
      .try_deserialize()
  }
}
