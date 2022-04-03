use crate::traveler::Traveler;
use display::DisplayDriver;
use nannou::prelude::Srgba;
use std::cell::RefCell;

pub struct Model {
  pub display_driver: DisplayDriver,
  pub travelers_colors: Vec<Srgba>,
  pub travelers_targets: Vec<usize>,
  // idea: create target trait and use it inside the travelers
  pub travelers: Vec<RefCell<Traveler>>,
}
