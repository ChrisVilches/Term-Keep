use crate::lines_from_file;
use rand::seq::SliceRandom;

pub fn random_tip() -> Option<String> {
  // TODO: This only works locally.
  match lines_from_file("./data/tips.txt") {
    Ok(tips) => tips.choose(&mut rand::thread_rng()).map(|s| s.to_string()),
    Err(_) => None,
  }
}
