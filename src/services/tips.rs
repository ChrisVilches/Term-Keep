use rand::seq::SliceRandom;

static TIPS_FILE: &str = include_str!("../../data/tips.txt");

#[must_use]
pub fn random_tip() -> Option<String> {
  TIPS_FILE
    .split('\n')
    .filter(|line| !line.is_empty())
    .collect::<Vec<&str>>()
    .choose(&mut rand::thread_rng())
    .map(|s| (*s).to_owned())
}
