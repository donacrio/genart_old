mod polar;

use nannou::prelude::*;
use polar::PolarPoint2;
// use rand::Rng;

const SIZE: u32 = 500;

fn main() {
    nannou::sketch(view).size(SIZE, SIZE).run();
}

// TODO -> Replace sketch with app
fn view(app: &App, frame: Frame) {
    // let mut rng = rand::thread_rng();
    let draw = app.draw();

    draw.background().color(WHITESMOKE);

    let win = app.window_rect();
    let win_pad = win.pad(0.05 * SIZE as f32);

    draw_circle(
        &draw,
        1.,
        DrawOptions {
            color: DARKGRAY,
            offset: None,
            weight: None,
            win_size: win_pad.w(),
        },
    );

    let control_points = vec![
        splines::Key::new(
            0.,
            Point2::from_polar(0., 0.),
            splines::Interpolation::Bezier(Point2::from_polar(1., deg_to_rad(45.))),
        ),
        splines::Key::new(
            1.,
            Point2::from_polar(1., deg_to_rad(160.)),
            splines::Interpolation::Bezier(
                2. * Point2::from_polar(1., deg_to_rad(160.))
                    - Point2::from_polar(1., deg_to_rad(100.)),
            ), // point symmetric -> 2*point-original
        ),
    ];
    draw_spline(
        &draw,
        control_points,
        DrawOptions {
            color: DARKGRAY,
            offset: None,
            weight: Some(2.),
            win_size: win_pad.w(),
        },
    );

    draw.to_frame(app, &frame).unwrap()
}

struct DrawOptions {
    pub color: Srgb<u8>,
    pub offset: Option<Point2>,
    pub weight: Option<f32>,
    pub win_size: f32,
}

fn draw_circle(draw: &Draw, radius: f32, options: DrawOptions) {
    let points = (0..=360).map(|i| {
        let radius = radius * options.win_size / 2.;
        let radian = deg_to_rad(i as f32);
        let point = Point2::from_polar(radius, radian);
        (point, options.color)
    });
    draw.polyline()
        .xy(options.offset.unwrap_or(pt2(0., 0.)))
        .weight(options.weight.unwrap_or(1.))
        .points_colored(points);
}

fn draw_spline(
    draw: &Draw,
    control_points: std::vec::Vec<splines::Key<f32, Point2>>,
    options: DrawOptions,
) {
    let spline = splines::Spline::from_vec(control_points);
    let points = (0..1000).map(|i| {
        let t = i as f32 / 1000.;
        let point = spline.sample(t).unwrap();
        let point = point * options.win_size / 2.;
        (point, options.color)
    });
    draw.polyline()
        .xy(options.offset.unwrap_or(pt2(0., 0.)))
        .weight(options.weight.unwrap_or(1.))
        .points_colored(points);
}
