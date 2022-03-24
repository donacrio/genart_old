use rand::prelude::{SeedableRng, SmallRng};
use signature::Signature;
use texture::TextureSaver;

pub struct Model {
  pub rng: SmallRng,
  pub texture_saver: TextureSaver,
  signature: Signature,
}

impl Model {
  pub fn new(config_path: &str, seed: u64, texture_saver: TextureSaver) -> Self {
    let rng = <SmallRng as SeedableRng>::seed_from_u64(seed);
    let signature = Signature::new(vec![
      signature::generate_git_hash(),
      signature::generate_file_hash(config_path),
      signature::generate_seed_hash(seed),
    ]);
    Self {
      rng,
      texture_saver,
      signature,
    }
  }

  pub fn signature(&self) -> &Signature {
    &self.signature
  }
}
