use config::{Config, ConfigError, File};
use nannou::prelude::{Point2, Srgb};
use serde::Deserialize;
use std::path::Path;

const SETTINGS_DEFAULT_PATH: &str = "configs/spinners/default.toml";

pub fn load_settings(path: Option<String>) -> Settings {
  let path = path.as_ref().map(|path| path.as_str()).unwrap_or_else(|| {
    println!("Could not find configuration file path. Using default configuration");
    SETTINGS_DEFAULT_PATH
  });

  match Settings::new(path) {
    Ok(settings) => settings,
    Err(err) => {
      println!("Encountered error: {}.\nProcess exited with code 1", err);
      std::process::exit(1);
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct WindowSettings {
  pub background_color: Srgb<u8>,
  pub height: u32,
  pub padding: i32,
  pub width: u32,
}

#[derive(Debug, Deserialize)]
pub struct SpinnerDrawingSettings {
  pub color: Srgb<u8>,
  pub point_weight: f32,
}

#[derive(Debug, Deserialize)]
pub struct SpinnerSettings {
  pub center: Point2,
  pub density: f32,
  pub density_factor: f32,
  pub drawing: SpinnerDrawingSettings,
  pub initial_points: i32,
  pub radius: f32,
  pub theta_increment: f32,
  pub theta_max: f32,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
  pub iterations: usize,
  pub window: WindowSettings,
  pub spinners: Vec<SpinnerSettings>,
}

impl Settings {
  pub fn new(path: &str) -> Result<Settings, ConfigError> {
    Config::builder()
      .add_source(File::from(Path::new(path)))
      .build()?
      .try_deserialize()
  }
}
