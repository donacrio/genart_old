use crate::traveler::Traveler;
use display::DisplayDriver;
use nannou::prelude::Srgba;
use std::sync::{Arc, Mutex};

pub struct Model {
  pub display_driver: DisplayDriver,
  pub travelers: Vec<Arc<Mutex<Traveler>>>,
  pub targets: Vec<Arc<Mutex<Traveler>>>,
  pub colors: Vec<Srgba>,
}
