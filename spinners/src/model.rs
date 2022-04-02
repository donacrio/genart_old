use display::DisplayDriver;
use rand::prelude::{SeedableRng, SmallRng};
use signature::Signature;

pub struct Model {
  pub display_driver: DisplayDriver,
  pub rng: SmallRng,
  signature: Signature,
}

impl Model {
  pub fn new(config_path: &str, seed: u64, display_driver: DisplayDriver) -> Self {
    let rng = <SmallRng as SeedableRng>::seed_from_u64(seed);
    let signature = Signature::new(vec![
      signature::generate_git_hash(),
      signature::generate_file_hash(config_path),
      signature::generate_seed_hash(seed),
    ]);
    Self {
      display_driver,
      rng,
      signature,
    }
  }

  pub fn signature(&self) -> &Signature {
    &self.signature
  }
}
