mod config;
mod constants;
mod params;

pub use crate::cli::config::Config;
use crate::cli::constants::*;
use getopts::Options;
use std::env;

pub fn run_cli() -> Result<Option<Config>, Box<dyn std::error::Error>> {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();
  let opts = get_opts();
  let matches = opts.parse(args)?;

  if matches.opt_present(HELP.long_name) {
    print_usage(&program, opts);
    return Ok(None);
  }

  let mut config = Config::load_env(matches.opt_str(ENV_FILEPATH.long_name))?;

  config.update_with_cli_params(matches)?;

  Ok(Some(config))
}

fn get_opts() -> Options {
  let mut opts = Options::new();
  for option in vec![
    ENV_FILEPATH,
    CENTERS,
    DENSITY_FACTOR,
    DENSITY_MAX,
    POINT_MAX,
    POINT_WEIGHT,
    THETA_MAX,
    THETA_STEP,
    WIN_SIZE,
  ] {
    opts.optopt(
      option.short_name,
      option.long_name,
      option.description,
      option.placeholder,
    );
  }
  for flag in vec![HELP] {
    opts.optflag(flag.short_name, flag.long_name, flag.description);
  }
  opts
}

fn print_usage(program: &str, opts: getopts::Options) {
  let brief = format!("Usage: {} [options]", program);
  print!("{}", opts.usage(&brief));
}
