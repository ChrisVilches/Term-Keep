use std::{cmp::Ordering, collections::HashMap};

use super::notes;
use crate::{models::note::Note, util};
use parking_lot::Mutex;
use rayon::prelude::*;

fn extract_all_tags(notes: &[Note], case_sensitive: bool) -> HashMap<String, usize> {
  let result = Mutex::new(HashMap::new());

  notes.into_par_iter().for_each(|note| {
    let content = if case_sensitive {
      note.content.clone()
    } else {
      note.content.to_lowercase()
    };

    for tag in util::tags::extract_tags_unique(&content) {
      *result.lock().entry(tag).or_default() += 1;
    }
  });

  result.into_inner()
}

fn cmp((s1, count1): &(String, usize), (s2, count2): &(String, usize)) -> Ordering {
  if count1.cmp(count2) == Ordering::Equal {
    s1.cmp(s2)
  } else {
    count2.cmp(count1)
  }
}

pub fn find_all(case_sensitive: bool) -> Vec<(String, usize)> {
  let mut tags: Vec<(String, usize)> =
    extract_all_tags(&notes::find_all_include_archived(), case_sensitive)
      .into_iter()
      .collect();
  tags.par_sort_unstable_by(cmp);
  tags
}

#[must_use]
pub fn find_notes_by_tag(tag_name: &str) -> Vec<Note> {
  let tag_name_lower = tag_name.to_lowercase();

  notes::find_all_include_archived()
    .into_par_iter()
    .filter(|n| {
      util::tags::extract_tags_non_unique(&n.content.to_lowercase()).contains(&tag_name_lower)
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case(vec!["#a", "#a #b", "#c"], vec![("a", 2), ("b", 1), ("c", 1)])]
  #[test_case(vec!["#a", "#a #c", "#c"], vec![("a", 2), ("c", 2)])]
  #[test_case(vec!["aaa", "bbb", "cc dd ee"], vec![])]
  #[test_case(vec!["this #タグ is supported", "tag #タグ and #text"], vec![("タグ", 2), ("text", 1)])]
  #[test_case(vec!["#tag", "#Tag #tag"], vec![("tag", 2), ("Tag", 1)])]
  #[test_case(vec![" #tag ", " #Tag #tag "], vec![("tag", 2), ("Tag", 1)]; "extra spaces")]
  #[test_case(vec![" #a #A "], vec![("A", 1), ("a", 1)])]
  fn test_extract_all_tags_case_sensitive(contents: Vec<&str>, result: Vec<(&str, usize)>) {
    let notes = contents
      .iter()
      .map(|c| {
        let mut note = Note::mock();
        note.content = (*c).to_owned();
        note
      })
      .collect::<Vec<Note>>();

    let map: HashMap<String, usize> = result.iter().map(|t| (t.0.to_owned(), t.1)).collect();
    assert_eq!(extract_all_tags(&notes, true), map);
  }

  #[test_case(vec![" #tag ", " #Tag #tag "], vec![("tag", 2)])]
  #[test_case(vec!["#a", "#A"], vec![("a", 2)])]
  #[test_case(vec![" #a #A "], vec![("a", 1)])]
  fn test_extract_all_tags(contents: Vec<&str>, result: Vec<(&str, usize)>) {
    let notes = contents
      .iter()
      .map(|c| {
        let mut note = Note::mock();
        note.content = (*c).to_owned();
        note
      })
      .collect::<Vec<Note>>();

    let map: HashMap<String, usize> = result.iter().map(|t| (t.0.to_owned(), t.1)).collect();
    assert_eq!(extract_all_tags(&notes, false), map);
  }
}
