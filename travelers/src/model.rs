use crate::traveler::Traveler;
use nannou::prelude::Vec2;
use nannou::prelude::{window, Draw};
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Model {
  pub _window: window::Id,
  pub _draw: Draw,
  pub travelers_target: HashMap<usize, usize>,
  pub travelers: Vec<RefCell<Traveler>>,
  pub points: Vec<Vec2>,
}
