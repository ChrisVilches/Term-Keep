use crate::{models::note::Note, services, util::note_fmt};
use colored::Colorize;

pub fn show_all(lowercase: bool) {
  let tags = services::tags::find_all(lowercase);

  for (tag_name, count) in tags {
    println!(
      "{} {}",
      format!("({} notes)", count).dimmed(),
      format!("#{}", tag_name).bold()
    );
  }
}

// TODO: Similar to the one in fuzzy_search. Try to recycle code.
fn format_result(note: &Note) -> String {
  vec![
    note_fmt::format_note_icons(note),
    note_fmt::format_note_summary(note),
  ]
  .iter()
  .filter(|s| !s.is_empty())
  .cloned()
  .collect::<Vec<String>>()
  .join(" ")
}

pub fn find_notes_by_tag(tag_name: &String) {
  let results = services::tags::find_notes_by_tag(tag_name);

  // TODO: This formatting/output is almost the same as the one for fuzzy search (except without scores)
  //       so try to recycle that code.

  println!(
    "{} results for {}",
    results.len().to_string().bold(),
    tag_name.bold()
  );

  if results.is_empty() {
    return;
  }

  println!();

  for note in &results {
    println!("{}", format_result(note));
  }
}
