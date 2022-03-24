pub struct Signature {
  hashes: Vec<String>,
}

impl Signature {
  pub fn new(hashes: Vec<String>) -> Self {
    Self { hashes }
  }

  pub fn generate_filename(&self) -> String {
    self.hashes.join("_")
  }

  pub fn generate_title(&self) -> String {
    self.hashes.join(" - ")
  }
}

pub fn generate_git_hash() -> String {
  last_git_commit::LastGitCommit::new()
    .build()
    .ok()
    .map(|commit| commit.id().short())
    .unwrap_or("".to_string())
}

pub fn generate_file_hash(path: &str) -> String {
  std::fs::read(path)
    .map(|bytes| sha256::digest_bytes(&bytes))
    .unwrap_or("".to_string())
}

pub fn generate_seed_hash(seed: u64) -> String {
  seed.to_string()
}
