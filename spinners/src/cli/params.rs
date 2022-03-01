pub struct CliOpt {
  pub short_name: &'static str,
  pub long_name: &'static str,
  pub description: &'static str,
  pub placeholder: &'static str,
  pub evar_name: &'static str,
}

impl CliOpt {
  pub const fn new(
    short_name: &'static str,
    long_name: &'static str,
    description: &'static str,
    placeholder: &'static str,
    evar_name: &'static str,
  ) -> CliOpt {
    CliOpt {
      short_name,
      long_name,
      description,
      placeholder,
      evar_name,
    }
  }
}

pub struct CliFlag {
  pub short_name: &'static str,
  pub long_name: &'static str,
  pub description: &'static str,
}

impl CliFlag {
  pub const fn new(
    short_name: &'static str,
    long_name: &'static str,
    description: &'static str,
  ) -> CliFlag {
    CliFlag {
      short_name,
      long_name,
      description,
    }
  }
}
