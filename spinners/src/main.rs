mod cli;
mod config;
mod spinner;

use crate::cli::parse_cli_args;
use crate::config::{load_config, Config};
use crate::spinner::Spinner;
use lazy_static::lazy_static;
use nannou::prelude::*;
use std::path::Path;

lazy_static! {
  static ref CONFIG: Config = load_config(parse_cli_args().config_file);
}

fn main() {
  nannou::sketch(view)
    .size(CONFIG.window.width, CONFIG.window.width)
    .loop_mode(LoopMode::NTimes {
      number_of_updates: 0,
    })
    .run();
}

fn view(app: &App, frame: Frame) {
  let draw = app.draw();

  let background_color: Srgb<f32> = CONFIG.window.background_color.into_format();
  draw.background().color(background_color);

  for spinner_config in &CONFIG.spinners {
    let spinner = Spinner::from(spinner_config);
    let points = spinner.compute_points();
    let options = DrawOptions {
      color: spinner_config.drawing.color.into_format(),
      point_weight: spinner_config.drawing.point_weight,
    };
    for point in &points {
      draw_point(&draw, point, &options);
    }
  }

  app
    .window(app.window_id())
    .unwrap()
    .capture_frame(Path::new(&format!(
      "./frames/spinners/{}/frame.jpeg",
      CONFIG.name
    )));
  draw.to_frame(app, &frame).unwrap();
}

struct DrawOptions {
  pub color: Srgb<u8>,
  pub point_weight: f32,
}

fn draw_point(draw: &Draw, point: &Point2, options: &DrawOptions) {
  draw
    .ellipse()
    .xy(*point)
    .w_h(options.point_weight, options.point_weight)
    .color(options.color);
}
