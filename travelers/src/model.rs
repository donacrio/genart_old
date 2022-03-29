use crate::traveler::Traveler;
use nannou::prelude::{Srgb, Vec2};
use std::cell::RefCell;
use std::collections::HashMap;
use texture::TextureSaver;

pub struct Model {
  pub _texture_saver: TextureSaver,
  pub travelers_colors: HashMap<usize, Srgb<u8>>,
  // idea: create target trait and use it inside the travelers
  pub travelers_target: HashMap<usize, usize>,
  pub travelers: Vec<RefCell<Traveler>>,
  pub points: Vec<Vec2>,
}
