use clap::Parser;

pub fn parse_cli_args() -> CliArgs {
  CliArgs::parse()
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
  /// Path to the Toml configuration file
  #[clap(short, long)]
  pub config_file: Option<String>,
}
