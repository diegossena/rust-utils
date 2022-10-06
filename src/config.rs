use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize)]
pub struct Config {
  pub uuid: String,
}

pub fn get() -> Config {
  let config_path = "conf.toml";
  let contents = match fs::read_to_string(config_path) {
    Ok(c) => c,
    Err(_) => {
      eprintln!("Could not read file `{}`", config_path);
      exit(1);
    }
  };
  match toml::from_str(&contents) {
    Ok(d) => d,
    Err(_) => {
      exit(1);
    }
  }
}
