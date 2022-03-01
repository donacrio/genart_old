mod cli;
mod model;
mod polar;
mod spinner;

use crate::cli::run_cli;
use crate::model::Model;
use crate::spinner::{Spinner, SpinnerConfig};
use nannou::app::LoopMode;
use nannou::prelude::*;
use std::path::Path;

fn main() {
    match run_cli() {
        Ok(config) => match config {
            Some(config) => nannou::app(model)
                .update(update)
                .loop_mode(LoopMode::NTimes {
                    number_of_updates: 0,
                })
                .size(config.win_size, config.win_size)
                .run(),
            None => std::process::exit(0),
        },
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    match run_cli() {
        Ok(config) => match config {
            Some(config) => {
                let mut spinners: Vec<Spinner> = Vec::new();
                let spinner_size =
                    1.5 * config.win_size as f32 * 0.95 / config.centers.len() as f32;

                for center in &config.centers {
                    let spinner = Spinner::new(SpinnerConfig::new(
                        *center,
                        config.density_max,
                        config.density_factor,
                        config.point_max,
                        spinner_size,
                        config.theta_step,
                    ));
                    spinners.push(spinner);
                }
                let mut model = Model::new(_window, config, spinners);
                let iterations =
                    (model.get_config().theta_max / model.get_config().theta_step) as i32;
                for _i in 1..iterations {
                    model.update();
                }
                model
            }
            None => std::process::exit(0),
        },
        Err(_) => std::process::exit(1),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    app.window(model._window)
        .unwrap()
        .capture_frame(Path::new("./frames/test.jpeg"));

    let draw = app.draw();

    draw.background().color(WHEAT);

    let options = DrawOptions {
        color: GRAY,
        weight: Some(model.get_config().point_weight),
    };

    for point in model.get_points() {
        draw_point(&draw, point, &options);
    }

    draw.to_frame(app, &frame).unwrap();
}

struct DrawOptions {
    pub color: Srgb<u8>,
    pub weight: Option<f32>,
}

fn draw_point(draw: &Draw, point: &Point2, options: &DrawOptions) {
    let weight = options.weight.unwrap_or(1.);
    draw.ellipse()
        .xy(*point)
        .w_h(weight, weight)
        .color(options.color);
}
