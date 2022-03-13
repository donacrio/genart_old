use crate::settings::WindowSettings;
use crate::spinner::Spinner;
use derive_getters::Getters;
use nannou::prelude::{window, Srgb};

#[derive(Getters)]
pub struct WindowConfig {
  background_color: Srgb<u8>,
  #[allow(unused)]
  height: u32,
  #[allow(unused)]
  padding: i32,
  #[allow(unused)]
  width: u32,
}

impl WindowConfig {
  pub fn new(background_color: Srgb<u8>, height: u32, padding: i32, width: u32) -> Self {
    Self {
      background_color,
      height,
      padding,
      width,
    }
  }
}

impl From<&WindowSettings> for WindowConfig {
  fn from(settings: &WindowSettings) -> Self {
    Self::new(
      settings.background_color.into_format(),
      settings.height,
      settings.padding,
      settings.width,
    )
  }
}

#[derive(Getters)]
pub struct Model {
  _window: window::Id,
  iteration: usize,
  spinners: Vec<Spinner>,
  window_config: WindowConfig,
}

impl Model {
  pub fn new(_window: window::Id, window_config: WindowConfig, spinners: Vec<Spinner>) -> Self {
    Self {
      _window,
      iteration: 0,
      spinners,
      window_config,
    }
  }

  pub fn update(&mut self) {
    for spinner in &mut self.spinners {
      spinner.update();
    }
  }

  pub fn set_spinners(&mut self, spinners: Vec<Spinner>) {
    self.spinners = spinners;
  }

  pub fn next_iteration(&mut self) {
    self.iteration += 1;
  }
}
