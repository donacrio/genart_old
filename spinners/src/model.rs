use crate::config::Config;
use crate::spinner::{Spinner, SpinnerInput};
use display::DisplayDriver;
use rand::prelude::{thread_rng, Rng};
use rand::prelude::{SeedableRng, SmallRng};
use signature::Signature;

pub struct Model {
  pub display_driver: DisplayDriver,
  pub spinners: Vec<Spinner>,
  pub iteration: usize,
  signature: Signature,
}

impl Model {
  pub fn new(config: &Config, config_path: &str, display_driver: DisplayDriver) -> Self {
    let seed: u64 = thread_rng().gen();
    let mut rng = <SmallRng as SeedableRng>::seed_from_u64(seed);
    let spinners = config
      .spinners
      .iter()
      .map(|spinner_config| {
        Spinner::from(SpinnerInput::new(
          spinner_config,
          &config.spinner_default_config,
          &mut rng,
        ))
      })
      .collect();
    let signature = Signature::new(vec![
      signature::generate_git_hash(),
      signature::generate_file_hash(config_path),
      signature::generate_seed_hash(seed),
    ]);
    Self {
      display_driver,
      spinners,
      iteration: 0,
      signature,
    }
  }

  pub fn signature(&self) -> &Signature {
    &self.signature
  }
}
