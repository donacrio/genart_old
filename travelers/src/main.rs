mod model;
mod traveler;

use crate::model::Model;
use crate::traveler::Traveler;
use nannou::prelude::*;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const MAX_TRAVELERS: usize = 100;
const MAX_VELOCITY: f32 = 2.;
const MAX_FORCE: f32 = 0.1;
const RADIUS: f32 = 10.;

fn main() {
    nannou::app(model).update(update).size(WIDTH, HEIGHT).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .size(WIDTH, HEIGHT)
        .build()
        .unwrap();

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
    let mut rng = rand::thread_rng();
    for i in 0..travelers.len() {
        let target = (i + rng.gen_range(1..travelers.len() / 2)) % travelers.len();
        travelers_target.insert(i, target);
    }

    Model {
        _window,
        _draw: nannou::Draw::new(),
        travelers,
        travelers_target,
        points: Vec::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for i in 0..model.travelers.len() {
        let target = model.travelers_target.get(&i).unwrap();
        let target = model.travelers.get(*target).unwrap().borrow();
        let mut traveler = model.travelers.get(i).unwrap().borrow_mut();
        traveler.seek(target.position + target.velocity);
        traveler.avoid_border(WIDTH as f32, HEIGHT as f32, RADIUS);
        traveler.update();
        draw_point(&model._draw, &target.position)
        // model.points.push(traveler.position);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = &model._draw;
    // draw.background().color(BLACK);

    // for point in &model.points {
    //     draw_point(&draw, point)
    // }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_point(draw: &Draw, point: &Vec2) {
    draw.ellipse().xy(*point).w_h(1., 1.).color(WHITE);
}
