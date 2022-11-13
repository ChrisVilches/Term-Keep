use crate::services;
use crate::util::note_fmt;
use crate::Note;
use colored::Colorize;

fn format_result(score: i64, note: &Note) -> String {
  format!(
    "{}\t{}",
    format!("(score {: >3})", score).purple().bold(),
    vec![
      note_fmt::format_note_icons(note),
      note_fmt::format_note_summary(note)
    ]
    .iter()
    .filter(|s| !s.is_empty())
    .cloned()
    .collect::<Vec<String>>()
    .join(" ")
  )
}

pub fn find_fuzzy(text: &String, archived: bool) {
  let results = services::notes::fuzzy_search(text, archived);

  println!(
    "{} results for {}",
    results.len().to_string().bold(),
    text.bold()
  );

  if results.is_empty() {
    return;
  }

  println!();

  for (score, note) in &results {
    println!("{}", format_result(*score, note));
  }
}
