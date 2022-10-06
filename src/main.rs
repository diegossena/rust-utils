mod config;

fn main() {
  let config = config::get();

  println!("{}", config.uuid);
}
