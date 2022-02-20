mod config;
mod model;
mod polar;
mod spinner;

use crate::config::CONFIG;
use crate::model::Model;
use crate::spinner::{Spinner, SpinnerConfig};
use nannou::app::LoopMode;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::NTimes {
            number_of_updates: 0,
        })
        .size(CONFIG.win_size, CONFIG.win_size)
        .run();
}

fn model(app: &App) -> Model {
    let spinner = Spinner::new(SpinnerConfig::new(
        CONFIG.theta_step,
        CONFIG.density_max,
        CONFIG.density_factor,
        CONFIG.point_max,
    ));
    let spinners = vec![spinner];
    let _window = app.new_window().view(view).build().unwrap();
    let mut model = Model::new(_window, spinners);

    for _i in 1..CONFIG.iterations {
        model.update();
    }
    model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    // app.window(model._window)
    //     .unwrap()
    //     .capture_frame(Path::new("./frames/test.jpeg"));

    let draw = app.draw();

    draw.background().color(WHEAT);

    let win = app.window_rect();
    let win_pad = win.pad(0.05 * CONFIG.win_size as f32);

    let options = DrawOptions {
        color: DARKGRAY,
        weight: Some(CONFIG.point_weight),
        win_size: win_pad.w(),
    };

    for point in model.get_points() {
        draw_point(&draw, point, &options);
    }

    draw.to_frame(app, &frame).unwrap();
}

struct DrawOptions {
    pub color: Srgb<u8>,
    pub weight: Option<f32>,
    pub win_size: f32,
}

fn draw_point(draw: &Draw, point: &Point2, options: &DrawOptions) {
    let weight = options.weight.unwrap_or(1.);
    let xy = *point * options.win_size / 2.;
    draw.ellipse()
        .xy(xy)
        .w_h(weight, weight)
        .color(options.color);
}
