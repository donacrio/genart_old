mod cli;
mod config;
mod model;
mod spinner;

use crate::cli::parse_cli_args;
use crate::config::{load_config, Config};
use crate::spinner::SpinnerDrawOptions;
use display::DisplayDriver;
use lazy_static::lazy_static;
use model::Model;
use nannou::prelude::*;

const CONFIG_DEFAULT_PATH: &str = "configs/spinners/default.toml";
const NADOU: &str = "Nadou";

lazy_static! {
  static ref CONFIG_PATH: String = parse_cli_args()
    .config_file
    .unwrap_or(CONFIG_DEFAULT_PATH.to_string());
  static ref CONFIG: Config = load_config(CONFIG_PATH.to_string());
  static ref NAME: String = parse_cli_args().name.unwrap_or("".to_string());
  // static ref N_ITERATIONS: usize = CONFIG
  //   .spinners
  //   .iter()
  //   .map(|spinner| {
  //     spinner
  //       .theta_increment
  //       .unwrap_or(CONFIG.spinner_default_config.theta_increment)
  //       / spinner
  //         .theta_max
  //         .unwrap_or(CONFIG.spinner_default_config.theta_max)
  //   })
  //   .max_by(|x, y| x.partial_cmp(y).unwrap())
  //   .unwrap() as usize;
    static ref N_ITERATIONS: usize = 500;

}

fn main() {
  nannou::app(model)
    .update(update)
    .loop_mode(LoopMode::NTimes {
      number_of_updates: *N_ITERATIONS,
    })
    .exit(exit)
    .run();
}

fn model(app: &App) -> Model {
  let texture_size = [CONFIG.window.width, CONFIG.window.height];
  let [win_w, win_h] = [texture_size[0] / 4, texture_size[1] / 4];
  let w_id = app
    .new_window()
    .size(win_w, win_h)
    .view(view)
    .build()
    .unwrap();
  let window = app.window(w_id).unwrap();

  let model = Model::new(
    &CONFIG,
    &CONFIG_PATH,
    DisplayDriver::new(&window, texture_size),
  );
  // Make sure the directory where we will save images to exists.
  std::fs::create_dir_all(&capture_directory(app, &model)).unwrap();

  model
}

fn update(app: &App, model: &mut Model, _update: Update) {
  let background_color: Srgb<f32> = CONFIG.window.background_color.into_format();
  // Reset the `draw` state.
  let draw = model.display_driver.draw();
  draw.reset();
  if app.elapsed_frames() == 0 {
    draw.background().color(background_color);
  }

  for spinner in model.spinners.iter_mut() {
    let points = spinner.compute_points(model.iteration);
    let options = &spinner.draw_options;
    for point in &points {
      draw_point(&draw, point, options);
    }
  }

  if model.iteration == *N_ITERATIONS - 1 {
    draw_signature(draw, &model);
  }

  model.iteration += 1;
  // Render our drawing to the texture.
  let window = app.main_window();
  model
    .display_driver
    .save(&window, capture_directory(app, model))
}

fn view(_app: &App, model: &Model, frame: Frame) {
  model.display_driver.render(frame);
}

fn exit(app: &App, model: Model) {
  let window = app.main_window();
  model.display_driver.wait(&window);
}

fn draw_point(draw: &Draw, point: &Point2, options: &SpinnerDrawOptions) {
  draw
    .ellipse()
    .xy(*point)
    .w_h(options.point_weight, options.point_weight)
    .color(options.color);
}

fn draw_signature(draw: &Draw, model: &Model) {
  // signing the frame
  let w = CONFIG.window.width as f32;
  let h = CONFIG.window.height as f32 * 0.05;
  let x = 0.;
  let y = -(CONFIG.window.height as f32) / 2. + h / 2.;

  let signature_rect = Rect::from_x_y_w_h(x, y, w, h);
  let nadou_rect = Rect::from_w_h(signature_rect.w() * 0.1, signature_rect.h() * 0.5)
    .align_right_of(signature_rect)
    .align_bottom_of(signature_rect);
  let hash_rect =
    Rect::from_w_h(signature_rect.w() * 0.5, signature_rect.h() * 0.5).above(signature_rect);

  draw
    .text(model.signature().generate_title().as_ref())
    .font_size(hash_rect.h() as u32 / 4)
    .x(hash_rect.x())
    .y(hash_rect.bottom())
    .wh(hash_rect.wh())
    .color(CONFIG.signature_color);

  draw
    .text(NADOU)
    .font_size(nadou_rect.h() as u32 / 3)
    .x(nadou_rect.x())
    .y(nadou_rect.top())
    .wh(nadou_rect.wh())
    .color(CONFIG.signature_color);
}

// The directory where we'll save the frames.
fn capture_directory(app: &nannou::app::App, model: &Model) -> std::path::PathBuf {
  let elapsed_frames = app.main_window().elapsed_frames();
  app
    .project_path()
    .expect("Could not locate project_path")
    .join("frames")
    .join("spinners")
    .join(NAME.as_str())
    .join(model.signature().generate_filename())
    .join(elapsed_frames.to_string())
}
