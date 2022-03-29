mod model;
mod traveler;

use crate::model::Model;
use crate::traveler::Traveler;
use nannou::prelude::*;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use texture::TextureSaver;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const MAX_TRAVELERS: usize = 1000;
const MAX_VELOCITY: f32 = 2.;
const MAX_FORCE: f32 = 0.05;
const RADIUS: f32 = 10.;

const DISPLAY_FACTOR: u32 = 1;

fn main() {
    nannou::app(model).update(update).run();
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

    let mut rng = rand::thread_rng();
    let mut travelers: Vec<RefCell<Traveler>> = Vec::new();
    for _ in 0..MAX_TRAVELERS {
        let theta = deg_to_rad(360. * rng.gen::<f32>());
        travelers.push(RefCell::new(Traveler::new(
            Vec2::splat(0.),
            Vec2::new(theta.cos(), theta.sin()),
            Vec2::splat(0.),
            MAX_VELOCITY,
            MAX_FORCE,
        )));
    }

    let mut travelers_target: HashMap<usize, usize> = HashMap::new();
    let mut travelers_colors: HashMap<usize, Srgb<u8>> = HashMap::new();
    let mut rng = rand::thread_rng();
    for i in 0..travelers.len() {
        let target = (i + rng.gen_range(1..travelers.len() / 2)) % travelers.len();
        travelers_target.insert(i, target);
        travelers_colors.insert(i, BROWN);
    }

    // Make sure the directory where we will save images to exists.
    std::fs::create_dir_all(&capture_directory(app)).unwrap();

    Model {
        _texture_saver: TextureSaver::new(&window, texture_size),
        travelers,
        travelers_colors,
        travelers_target,
        points: Vec::new(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Reset the `draw` state.
    let draw = model._texture_saver.draw();
    draw.reset();
    if app.elapsed_frames() == 0 {
        draw.background().color(Srgb::<u8>::new(245, 222, 179));
    }

    for i in 0..model.travelers.len() {
        let target = model.travelers_target.get(&i).unwrap();
        let target = model.travelers.get(*target).unwrap().borrow();
        let mut traveler = model.travelers.get(i).unwrap().borrow_mut();
        let color = model.travelers_colors.get(&i).unwrap();
        traveler.seek(target.position + target.velocity);
        traveler.avoid_border(WIDTH as f32, HEIGHT as f32, RADIUS);
        traveler.update();
        traveler.correct_position(WIDTH as f32, HEIGHT as f32);
        traveler.draw(&draw, &target.position, color);
    }
    // Render our drawing to the texture.
    let window = app.main_window();
    model._texture_saver.save(&window, capture_directory(app))
}

fn view(_app: &App, model: &Model, frame: Frame) {
    model._texture_saver.render(frame);
}

// The directory where we'll save the frames.
fn capture_directory(app: &nannou::app::App) -> std::path::PathBuf {
    app.project_path()
        .expect("Could not locate project_path")
        .join("frames")
        .join("travelers")
}
