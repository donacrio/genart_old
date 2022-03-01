//! Config struct used to parse environment variable and CLI parameters.

use crate::cli::constants::*;
use getopts::Matches;
use nannou::prelude::{pt2, Point2};
use std::env;
use std::error::Error;

pub struct Config {
  pub centers: Vec<Point2>,
  pub density_factor: f32,
  pub density_max: f32,
  pub point_max: i32,
  pub point_weight: f32,
  pub theta_max: f32,
  pub theta_step: f32,
  pub win_size: u32,
}

impl Config {
  pub fn load_env(env_filepath: Option<String>) -> Result<Config, Box<dyn Error>> {
    match env_filepath {
      Some(env_file_path) => dotenv::from_filename(env_file_path),
      None => dotenv::dotenv(),
    }?;

    // Parse environment variables here
    let centers = env::var(CENTERS.evar_name)?
      .parse::<String>()?
      .split(";")
      .map(|point| {
        let coordinates: Vec<String> = point
          .split(',')
          .map(|el| el.to_string())
          .collect::<Vec<String>>();
        if coordinates.len() != 2 {
          return None;
        }
        let x = coordinates[0].parse::<f32>().unwrap();
        let y = coordinates[1].parse::<f32>().unwrap();
        Some(pt2(x, y))
      })
      .filter_map(|e| e)
      .collect::<Vec<Point2>>();
    let density_factor: f32 = env::var(DENSITY_FACTOR.evar_name)?.parse()?;
    let density_max: f32 = env::var(DENSITY_MAX.evar_name)?.parse()?;
    let point_max: i32 = env::var(POINT_MAX.evar_name)?.parse()?;
    let point_weight: f32 = env::var(POINT_WEIGHT.evar_name)?.parse()?;
    let theta_max: f32 = env::var(THETA_MAX.evar_name)?.parse()?;
    let theta_step: f32 = env::var(THETA_STEP.evar_name)?.parse()?;
    let win_size: u32 = env::var(WIN_SIZE.evar_name)?.parse()?;

    Ok(Config {
      centers,
      density_factor,
      density_max,
      point_max,
      point_weight,
      theta_max,
      theta_step,
      win_size,
    })
  }
}

impl Config {
  /// Override the config with the given CLI parameters.
  pub fn update_with_cli_params(&mut self, matches: Matches) -> Result<(), Box<dyn Error>> {
    // Parse cli parameters here
    if let Some(centers) = matches.opt_str(CENTERS.long_name) {
      self.centers = centers
        .parse::<String>()?
        .split(";")
        .map(|point| {
          let coordinates: Vec<String> = point
            .split(',')
            .map(|el| el.to_string())
            .collect::<Vec<String>>();
          if coordinates.len() != 2 {
            return None;
          }
          let x = coordinates[0].parse::<f32>().unwrap();
          let y = coordinates[1].parse::<f32>().unwrap();
          Some(pt2(x, y))
        })
        .filter_map(|e| e)
        .collect::<Vec<Point2>>();
    };
    if let Some(density_factor) = matches.opt_str(DENSITY_FACTOR.long_name) {
      self.density_factor = density_factor.parse()?;
    };
    if let Some(density_max) = matches.opt_str(DENSITY_MAX.long_name) {
      self.density_max = density_max.parse()?;
    };
    if let Some(point_max) = matches.opt_str(POINT_MAX.long_name) {
      self.point_max = point_max.parse()?;
    };
    if let Some(point_weight) = matches.opt_str(POINT_WEIGHT.long_name) {
      self.point_weight = point_weight.parse()?;
    };
    if let Some(theta_max) = matches.opt_str(THETA_MAX.long_name) {
      self.theta_max = theta_max.parse()?;
    };
    if let Some(theta_step) = matches.opt_str(THETA_STEP.long_name) {
      self.theta_step = theta_step.parse()?;
    };
    if let Some(win_size) = matches.opt_str(WIN_SIZE.long_name) {
      self.win_size = win_size.parse()?;
    };

    Ok(())
  }
}
