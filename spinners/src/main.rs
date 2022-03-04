mod cli;
mod model;
mod polar;
mod settings;
mod spinner;

use crate::cli::parse_cli_args;
use crate::model::{Model, WindowConfig};
use crate::spinner::Spinner;
use lazy_static::lazy_static;
use nannou::app::LoopMode;
use nannou::prelude::*;
use settings::{load_settings, Settings};
use std::path::Path;

lazy_static! {
    static ref SETTINGS: Settings = load_settings(parse_cli_args().config_file);
}

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::NTimes {
            number_of_updates: SETTINGS.iterations,
        })
        .run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .size(SETTINGS.window.width, SETTINGS.window.height)
        .build()
        .unwrap();
    let model = Model::new(_window, WindowConfig::from(&SETTINGS.window), Vec::new());
    model
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.set_spinners(Vec::new());
    let mut spinners: Vec<Spinner> = Vec::new();
    for spinner_settings in &SETTINGS.spinners {
        let spinner = Spinner::from(spinner_settings);
        spinners.push(spinner);
    }
    model.set_spinners(spinners);
    model.update();
    model.next_iteration();
}

fn view(app: &App, model: &Model, frame: Frame) {
    app.window(*model._window())
        .unwrap()
        .capture_frame(Path::new(&format!(
            "./frames/frame_{}.jpeg",
            model.iteration()
        )));

    let draw = app.draw();

    draw.background()
        .color(*model.window_config().background_color());

    for spinner in model.spinners() {
        let options = DrawOptions {
            color: *spinner.config().drawing().color(),
            point_weight: *spinner.config().drawing().point_weight(),
        };
        for point in spinner.state().points() {
            draw_point(&draw, point, &options);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

struct DrawOptions {
    pub color: Srgb<u8>,
    pub point_weight: f32,
}

fn draw_point(draw: &Draw, point: &Point2, options: &DrawOptions) {
    draw.ellipse()
        .xy(*point)
        .w_h(options.point_weight, options.point_weight)
        .color(options.color);
}
