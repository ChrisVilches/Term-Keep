use colored::Colorize;

use crate::{
  models::note::Note,
  services,
  util::{self, note_fmt},
};

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

// TODO: This format is not very clean. When there are icons, they ruin the layout a bit.
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

pub fn find_notes_by_tag(tag_name: &str) {
  let results = services::tags::find_notes_by_tag(tag_name);

  println!(
    "{}",
    util::search::format_search_results(tag_name, &results, format_result)
  );
}