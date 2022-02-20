mod cli;
mod config;
mod spinner;

use crate::cli::parse_cli_args;
use crate::config::{load_config, Config};
use crate::spinner::{Spinner, SpinnerInput};
use lazy_static::lazy_static;
use nannou::prelude::*;
use texture::TextureSaver;

lazy_static! {
  static ref CONFIG: Config = load_config(parse_cli_args().config_file);
  static ref NAME: String = parse_cli_args().name.unwrap_or(CONFIG.name.clone());
}

fn main() {
  nannou::app(model)
    .update(update)
    .exit(exit)
    .loop_mode(LoopMode::NTimes {
      number_of_updates: CONFIG.iterations,
    })
    .run();
}

fn model(app: &App) -> TextureSaver {
  // Write to a 4K UHD texture.
  let texture_size = [CONFIG.window.width, CONFIG.window.height];

  // Create the window.
  let [win_w, win_h] = [texture_size[0] / 4, texture_size[1] / 4];
  let w_id = app
    .new_window()
    .size(win_w, win_h)
    .title(&CONFIG.name)
    .view(view)
    .build()
    .unwrap();
  let window = app.window(w_id).unwrap();

  // Make sure the directory where we will save images to exists.
  std::fs::create_dir_all(&capture_directory(app)).unwrap();

  TextureSaver::new(&window, texture_size)
}

fn update(app: &App, model: &mut TextureSaver, _update: Update) {
  // Reset the `draw` state.
  let draw = model.draw();
  draw.reset();

  let background_color: Srgb<f32> = CONFIG.window.background_color.into_format();
  draw.background().color(background_color);

  for spinner_config in &CONFIG.spinners {
    let spinner = Spinner::from(SpinnerInput::new(
      spinner_config,
      &CONFIG.spinner_default_config,
    ));
    let points = spinner.compute_points();
    let options = DrawOptions {
      color: spinner_config
        .drawing
        .as_ref()
        .unwrap_or(&CONFIG.spinner_default_config.drawing)
        .color
        .into_format(),
      point_weight: spinner_config
        .drawing
        .as_ref()
        .unwrap_or(&CONFIG.spinner_default_config.drawing)
        .point_weight,
    };
    for point in &points {
      draw_point(&draw, point, &options);
    }
  }

  // Render our drawing to the texture.
  let window = app.main_window();
  model.save(&window, capture_directory(app))
}

fn view(_app: &App, model: &TextureSaver, frame: Frame) {
  model.render(frame);
}

fn exit(app: &App, model: TextureSaver) {
  let window = app.main_window();
  model.wait(&window);
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

// The directory where we'll save the frames.
fn capture_directory(app: &nannou::app::App) -> std::path::PathBuf {
  app
    .project_path()
    .expect("Could not locate project_path")
    .join("frames")
    .join("spinners")
    .join(NAME.as_str())
}
