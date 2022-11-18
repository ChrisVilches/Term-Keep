use std::{
  cmp::Ordering,
  collections::{HashMap, HashSet},
  sync::Mutex,
};

use crate::models::note::Note;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;

use super::notes;

lazy_static! {
  // TODO: Tag regex is still very simple. I need to add more restrictions to it.
  static ref TAG_REGEX: Regex = Regex::new(r"^#([^\s^#]+)$").unwrap();
  static ref WHITESPACE_REGEX: Regex = Regex::new(r"\s+").unwrap();
}

fn extract_tags(text: &str) -> HashSet<String> {
  WHITESPACE_REGEX
    .split(text)
    .into_iter()
    .filter(|w| TAG_REGEX.is_match(w))
    .map(|s| s[1..].to_owned())
    .collect()
}

fn extract_all_tags(notes: &Vec<Note>, lowercase: bool) -> HashMap<String, usize> {
  let result = Mutex::new(HashMap::new());

  notes.into_par_iter().for_each(|note| {
    let content = if lowercase {
      note.content.to_lowercase()
    } else {
      note.content.clone()
    };

    let tags = extract_tags(&content);
    let mut map = result.lock().unwrap();

    for tag in tags {
      *map.entry(tag).or_default() += 1;
    }
  });

  result.into_inner().unwrap()
}

fn cmp((s1, count1): &(String, usize), (s2, count2): &(String, usize)) -> Ordering {
  if count1.cmp(count2) == Ordering::Equal {
    s1.cmp(s2)
  } else {
    count2.cmp(count1)
  }
}

pub fn find_all(lowercase: bool) -> Vec<(String, usize)> {
  let mut tags: Vec<(String, usize)> =
    extract_all_tags(&notes::find_all_include_archived(), lowercase)
      .into_iter()
      .collect();
  tags.par_sort_unstable_by(cmp);
  tags
}

pub fn find_notes_by_tag(tag_name: &str) -> Vec<Note> {
  let tag_name_lower = tag_name.to_lowercase();

  notes::find_all_include_archived()
    .into_par_iter()
    .filter(|n| extract_tags(&n.content.to_lowercase()).contains(&tag_name_lower))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case("this #text is a tag", vec!["text"])]
  #[test_case("#a #b", vec!["a", "b"])]
  #[test_case("#a #a  #a #a", vec!["a"])]
  #[test_case("some animals #🐊 #🐌🐌🐌", vec!["🐊", "🐌🐌🐌"])]
  #[test_case("some #tag\nwith spaces", vec!["tag"])]
  #[test_case("two tags #tag and #Tag", vec!["tag", "Tag"])]
  #[test_case("invalid tags # end", vec![])]
  #[test_case("invalid tags #aaaa#bbbb end", vec![])]
  #[test_case("japanese space #　", vec![])]
  #[test_case("some #tags with\n#newlines", vec!["tags", "newlines"])]
  fn test_extract_tags(text: &str, result: Vec<&str>) {
    let set: HashSet<String> = result.iter().map(|s| (*s).to_owned()).collect();
    assert_eq!(extract_tags(text), set);
  }

  #[test_case(vec!["#a", "#a #b", "#c"], vec![("a", 2), ("b", 1), ("c", 1)])]
  #[test_case(vec!["#a", "#a #c", "#c"], vec![("a", 2), ("c", 2)])]
  #[test_case(vec!["aaa", "bbb", "cc dd ee"], vec![])]
  #[test_case(vec!["this #タグ is supported", "tag #タグ and #text"], vec![("タグ", 2), ("text", 1)])]
  #[test_case(vec!["#tag", "#Tag #tag"], vec![("tag", 2), ("Tag", 1)])]
  #[test_case(vec![" #tag ", " #Tag #tag "], vec![("tag", 2), ("Tag", 1)]; "extra spaces")]
  #[test_case(vec![" #a #A "], vec![("A", 1), ("a", 1)])]
  fn test_extract_all_tags(contents: Vec<&str>, result: Vec<(&str, usize)>) {
    let notes = contents
      .iter()
      .map(|c| {
        let mut note = Note::mock();
        note.content = (*c).to_owned();
        note
      })
      .collect();

    let map: HashMap<String, usize> = result.iter().map(|t| (t.0.to_owned(), t.1)).collect();
    assert_eq!(extract_all_tags(&notes, false), map);
  }

  #[test_case(vec![" #tag ", " #Tag #tag "], vec![("tag", 2)])]
  #[test_case(vec!["#a", "#A"], vec![("a", 2)])]
  #[test_case(vec![" #a #A "], vec![("a", 1)])]
  fn test_extract_all_tags_lowercase(contents: Vec<&str>, result: Vec<(&str, usize)>) {
    let notes = contents
      .iter()
      .map(|c| {
        let mut note = Note::mock();
        note.content = (*c).to_owned();
        note
      })
      .collect();

    let map: HashMap<String, usize> = result.iter().map(|t| (t.0.to_owned(), t.1)).collect();
    assert_eq!(extract_all_tags(&notes, true), map);
  }

  #[test_case("#aaa#bbb", false)]
  #[test_case("#fffff", true)]
  #[test_case("#b", true)]
  #[test_case("#", false; "only hashtag")]
  #[test_case("", false; "empty")]
  #[test_case("#cccc ", false)]
  #[test_case(" #ddd", false)]
  #[test_case(" eeeee", false)]
  fn test_tag_regex(text: &str, result: bool) {
    assert_eq!(TAG_REGEX.is_match(text), result);
  }
}
