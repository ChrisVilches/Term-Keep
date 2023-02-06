use crate::models::note::Note;
use crate::util::note_fmt;
use crate::{services, util};
use colored::Colorize;

// TODO: This format is not very clean. When there are icons, they ruin the layout a bit.
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

pub fn find_fuzzy(text: &str) {
  let results = services::notes::fuzzy_search(text);

  println!(
    "{}",
    util::search::format_search_results(text, &results, |tuple| format_result(tuple.0, &tuple.1))
  );
}
