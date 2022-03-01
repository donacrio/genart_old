use crate::cli::params::{CliFlag, CliOpt};

pub const ENV_FILEPATH: CliOpt = CliOpt::new(
  "",
  "env-file",
  "set .env filepath",
  "FILEPATH",
  "ENV_FILEPATH",
);

pub const HELP: CliFlag = CliFlag::new("h", "help", "display this message");

pub const CENTERS: CliOpt = CliOpt::new(
  "c",
  "centers",
  "define a new spinner with it's center",
  "CENTERS",
  "CENTERS",
);

pub const DENSITY_FACTOR: CliOpt = CliOpt::new(
  "d",
  "density-factor",
  "set the density factor",
  "DENSITY_FACTOR",
  "DENSITY_FACTOR",
);

pub const DENSITY_MAX: CliOpt = CliOpt::new(
  "D",
  "density-max",
  "set the maximum density",
  "DENSITY_MAX",
  "DENSITY_MAX",
);

pub const POINT_MAX: CliOpt = CliOpt::new(
  "P",
  "point-max",
  "set the maximum number of points at every iteration",
  "POINT_MAX",
  "POINT_MAX",
);

pub const POINT_WEIGHT: CliOpt = CliOpt::new(
  "w",
  "point-weight",
  "set weight of every point",
  "POINT_WEIGHT",
  "POINT_WEIGHT",
);

pub const THETA_MAX: CliOpt = CliOpt::new(
  "T",
  "theta-max",
  "set the angular maximum rotation in radian",
  "THETA_MAX",
  "THETA_MAX",
);

pub const THETA_STEP: CliOpt = CliOpt::new(
  "t",
  "theta-incr",
  "set the angular increment in radian",
  "THETA_STEP",
  "THETA_STEP",
);

pub const WIN_SIZE: CliOpt = CliOpt::new("s", "size", "set the frame size", "WIN_SIZE", "WIN_SIZE");
