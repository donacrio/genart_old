pub const CONFIG: Config = Config {
  density_factor: 0.9,
  density_max: 1.,
  iterations: 1000,
  point_max: 1000,
  point_weight: 0.4,
  theta_step: 0.01,
  win_size: 500,
};

pub struct Config {
  pub density_factor: f32,
  pub density_max: f32,
  pub iterations: i32,
  pub point_max: i32,
  pub point_weight: f32,
  pub theta_step: f32,
  pub win_size: u32,
}
