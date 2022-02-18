use nannou::prelude::Point2;

pub trait PolarPoint2 {
  fn from_polar(radius: f32, angle: f32) -> Self;
  fn get_polar(&self) -> (f32, f32);
}

impl PolarPoint2 for Point2 {
  fn from_polar(radius: f32, radian: f32) -> Self {
    Self::new(radius * radian.cos(), radius * radian.sin())
  }
  fn get_polar(&self) -> (f32, f32) {
    let radius = (self.x.powf(2.) + self.y.powf(2.)).sqrt();
    let angle = (self.y / self.x).atan();
    (radius, angle)
  }
}
