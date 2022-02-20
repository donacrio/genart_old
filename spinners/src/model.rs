use crate::spinner::Spinner;
use nannou::prelude::{window, Point2};

pub struct Model {
  pub _window: window::Id,
  pub spinners: Vec<Spinner>,
}

impl Model {
  pub fn new(_window: window::Id, spinners: Vec<Spinner>) -> Model {
    Model { _window, spinners }
  }

  pub fn update(&mut self) {
    for spinner in &mut self.spinners {
      spinner.update();
    }
  }

  pub fn get_points(&self) -> Vec<&Point2> {
    let mut points = Vec::<&Point2>::new();
    for spinner in &self.spinners {
      points.extend(spinner.get_points());
    }
    points
  }
}
