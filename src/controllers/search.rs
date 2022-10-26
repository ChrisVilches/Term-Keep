use crate::services;
use crate::util::note_fmt;
use crate::Note;
use colored::Colorize;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::cmp::Ordering;

fn cmp((score1, n1): &(i64, &Note), (score2, _): &(i64, &Note)) -> Ordering {
  let ord = score2.cmp(score1);

  if ord == Ordering::Equal {
    if n1.pinned {
      Ordering::Less
    } else {
      Ordering::Greater
    }
  } else {
    ord
  }
}

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
  let notes: Vec<Note> = services::notes::find_all(archived);

  let matcher = SkimMatcherV2::default();

  let mut results: Vec<(i64, &Note)> = notes
    .iter()
    .map(|note| (matcher.fuzzy_match(&note.content, text).unwrap_or(0), note))
    .filter(|pair| pair.0 > 0)
    .collect();

  results.sort_by(cmp);

  println!(
    "{} results for {}",
    results.len().to_string().bold(),
    text.bold()
  );
  println!();

  for (score, note) in &results {
    println!("{}", format_result(*score, note));
  }
}
