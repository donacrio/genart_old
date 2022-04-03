mod model;
mod strategy;
mod traveler;

use crate::model::Model;
use crate::strategy::{apply_strategy, Strategy};
use crate::traveler::Traveler;
use display::DisplayDriver;
use nannou::prelude::*;
use rand::Rng;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};

const WIDTH: u32 = 1470;
const HEIGHT: u32 = 1470;
const N_TRAVELERS: usize = 200;
const MAX_VELOCITY: f32 = 3.;
const MAX_FORCE: f32 = 0.1;
const MAX_POINTS: u32 = 100;
const SEEK_RANGE: usize = 7;
const DISPLAY_FACTOR: u32 = 2;
const COLORS: [(u8, u8, u8); 15] = [
    (17, 21, 46),
    (19, 29, 120),
    (56, 182, 255),
    (249, 214, 57),
    (252, 183, 34),
    (255, 149, 10),
    (254, 96, 75),
    (216, 49, 91),
    (113, 72, 45),
    (253, 150, 53),
    (255, 243, 133),
    (252, 183, 34),
    (173, 47, 36),
    (183, 122, 55),
    (255, 222, 179),
];

fn main() {
    nannou::app(model).update(update).exit(exit).run();
}

fn model(app: &App) -> Model {
    let texture_size = [WIDTH, HEIGHT];
    // Create the window.
    let [win_w, win_h] = [
        texture_size[0] / DISPLAY_FACTOR,
        texture_size[1] / DISPLAY_FACTOR,
    ];
    let w_id = app
        .new_window()
        .size(win_w, win_h)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(w_id).unwrap();

    let mut travelers = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..N_TRAVELERS {
        let theta = 2. * PI * rng.gen::<f32>();
        travelers.push(Arc::new(Mutex::new(Traveler::new(
            Vec2::splat(0.),
            Vec2::new(theta.cos(), theta.sin()) * MAX_VELOCITY,
            Vec2::splat(0.),
            (rng.gen::<f32>() * MAX_POINTS as f32) as usize,
            MAX_VELOCITY,
            MAX_FORCE,
        ))));
    }

    let mut targets = Vec::new();
    let mut colors = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..N_TRAVELERS {
        let target = (i + rng.gen_range(1..SEEK_RANGE)) % N_TRAVELERS;
        let target = travelers.get(target).unwrap();
        targets.push(Arc::clone(target));
        let color: usize = rng.gen_range(0..10);
        let color = COLORS[color];
        let color = Srgba::new(
            color.0 as f32 / 255.,
            color.1 as f32 / 255.,
            color.2 as f32 / 255.,
            1.,
        );
        colors.push(color);
    }

    // Make sure the directory where we will save images to exists.
    std::fs::create_dir_all(&capture_directory(app)).unwrap();

    Model {
        display_driver: DisplayDriver::new(&window, texture_size),
        travelers,
        targets,
        colors,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Reset the `draw` state.
    let draw = model.display_driver.draw();
    draw.reset();
    if app.elapsed_frames() == 0 {
        draw.background().color(WHEAT);
    }

    let center = Vec2::new(0., 0.);
    for (index, traveler) in model.travelers.iter().enumerate() {
        let mut traveler = traveler.lock().unwrap();
        let target = model.targets.get(index).unwrap();
        let target = target.lock().unwrap();
        apply_strategy(&mut traveler, &target.position, Strategy::SEEK, MAX_FORCE);
        let gravity = get_inverse_gravity(&traveler.position, &center, MAX_FORCE);
        apply_strategy(&mut traveler, &center, Strategy::SEEK, gravity);
        traveler.update();
        let color = model.colors.get(index).unwrap();
        draw_traveler(&traveler, &target.position, &draw, color);
    }

    // Render our drawing to the texture.
    let window = app.main_window();
    model.display_driver.save(&window, capture_directory(app));
}

fn view(_app: &App, model: &Model, frame: Frame) {
    model.display_driver.render(frame);
}

fn exit(app: &App, model: Model) {
    let window = app.main_window();
    model.display_driver.wait(&window);
}

// The directory where we'll save the frames.
fn capture_directory(app: &nannou::app::App) -> std::path::PathBuf {
    let elapsed_frames = app.main_window().elapsed_frames();
    app.project_path()
        .expect("Could not locate project_path")
        .join("frames")
        .join("travelers")
        .join(elapsed_frames.to_string())
}

pub fn draw_traveler(traveler: &Traveler, target: &Vec2, draw: &Draw, color: &Srgba) {
    let mut rng = rand::thread_rng();
    let middle = (traveler.position + *target) / 2.;
    let direction = *target - traveler.position;
    for _ in 0..traveler.n_points {
        let theta = 2. * PI * rng.gen::<f32>();
        let point = middle + theta.sin() / 2. * direction;
        draw.ellipse().xy(point).w_h(1., 1.).color(*color);
        let point = middle - theta.sin() / 2. * direction;
        draw.ellipse().xy(point).w_h(1., 1.).color(*color);
    }
}

fn get_inverse_gravity(a: &Vec2, b: &Vec2, max_force: f32) -> f32 {
    let distance = (*b - *a).length();
    let limit = WIDTH.min(HEIGHT) as f32;
    if distance < limit / 2. {
        return 0.;
    } else {
        return (distance / limit).min(max_force);
    }
}
