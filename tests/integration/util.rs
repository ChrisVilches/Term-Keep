use rand::Rng;

pub fn randomize_cases(s: &str) -> String {
  s.chars()
    .map(|c| {
      if rand::thread_rng().gen_bool(0.5) {
        c.to_ascii_lowercase()
      } else {
        c.to_ascii_uppercase()
      }
    })
    .collect::<String>()
}
