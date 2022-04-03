mod model;
mod traveler;

use crate::model::Model;
use crate::traveler::Traveler;
use display::DisplayDriver;
use nannou::prelude::*;
use rand::Rng;
use std::cell::RefCell;

const WIDTH: u32 = 3840;
const HEIGHT: u32 = 3840;
const N_TRAVELERS: usize = 500;
const MAX_VELOCITY: f32 = 4.;
const MAX_FORCE: f32 = 0.5;
const RADIUS: f32 = 300.;
const MAX_POINTS: u32 = 500;
const DISPLAY_FACTOR: u32 = 4;
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

    let mut rng = rand::thread_rng();
    let mut travelers = Vec::new();
    for _ in 0..N_TRAVELERS {
        let theta = deg_to_rad(360. * rng.gen::<f32>());
        travelers.push(RefCell::new(Traveler::new(
            Vec2::splat(0.),
            Vec2::new(theta.cos(), theta.sin()) * MAX_VELOCITY * rng.gen::<f32>(),
            Vec2::splat(0.),
            (rng.gen::<f32>() * MAX_POINTS as f32) as usize,
            MAX_VELOCITY,
            MAX_FORCE,
        )));
    }

    let mut travelers_targets: Vec<usize> = Vec::new();
    let mut travelers_colors: Vec<Srgba> = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..N_TRAVELERS {
        // let target = (i + rng.gen_range(1..travelers.len())) % travelers.len();
        let target = (i + rng.gen_range(1..5)) % N_TRAVELERS;
        travelers_targets.push(target);
        let color: usize = rng.gen_range(0..10);
        let color = COLORS[color];
        let color = Srgba::new(
            color.0 as f32 / 255.,
            color.1 as f32 / 255.,
            color.2 as f32 / 255.,
            1.,
        );
        travelers_colors.push(color);
    }

    // Make sure the directory where we will save images to exists.
    std::fs::create_dir_all(&capture_directory(app)).unwrap();

    Model {
        display_driver: DisplayDriver::new(&window, texture_size),
        travelers,
        travelers_colors,
        travelers_targets,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Reset the `draw` state.
    let draw = model.display_driver.draw();

    draw.reset();
    if app.elapsed_frames() == 0 {
        draw.background().color(WHEAT);
    }

    model
        .travelers
        .iter()
        .enumerate()
        .for_each(|(index, traveler)| {
            let mut traveler = traveler.borrow_mut();
            let target = model.travelers_targets.get(index).unwrap();
            let target = model.travelers.get(*target).unwrap().borrow();
            let color = model.travelers_colors.get(index).unwrap();
            traveler.seek(target.position + target.velocity);
            traveler.avoid_border(WIDTH as f32, HEIGHT as f32, RADIUS);
            traveler.update();
            traveler.draw(&draw, &target.position, color)
        });

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
