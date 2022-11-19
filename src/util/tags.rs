use fancy_regex::Captures;
use lazy_static::lazy_static;
use std::collections::HashSet;

use super::strings::highlight;

lazy_static! {
  // TODO: The look-behind/ahead works. Now all I have to do is improve the content of the () in the regex (the tag name matching).
  //       For example, avoid symbols and weird things. But do allow Japanese and other languages.
  //       I currently added a few symbols, but it's not enough.
  pub static ref TAG_REGEX: fancy_regex::Regex = fancy_regex::Regex::new(r"(?<=\s|^)#([^[\s#\.\,\)\(\'\&\%\$)]]+)(?=\s|$)").unwrap();
}

pub fn format_text(s: &str) -> String {
  TAG_REGEX
    .replace_all(s, |c: &Captures| highlight(&format!("#{}", &c[1])))
    .into_owned()
}

pub fn extract_tags_non_unique(text: &str) -> Vec<String> {
  TAG_REGEX
    .captures_iter(text)
    .map(|cap| cap.unwrap()[1].to_owned())
    .collect()
}

pub fn extract_tags_unique(text: &str) -> HashSet<String> {
  extract_tags_non_unique(text).into_iter().collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case("#aaa#bbb", false)]
  #[test_case("#fffff", true)]
  #[test_case("#b", true)]
  #[test_case("#", false; "only hashtag")]
  #[test_case("", false; "empty")]
  #[test_case("#cccc ", true)]
  #[test_case(" #ddd", true)]
  #[test_case(" eeeee", false)]
  fn test_tag_regex(text: &str, result: bool) {
    assert_eq!(TAG_REGEX.is_match(text).unwrap(), result);
  }

  #[test_case("this #text is a tag", vec!["text"])]
  #[test_case("#a #b", vec!["a", "b"])]
  #[test_case("#a #a  #a #a", vec!["a", "a", "a", "a"])]
  #[test_case("#first #period. #last", vec!["first", "last"])]
  #[test_case("some animals #🐊 #🐌🐌🐌", vec!["🐊", "🐌🐌🐌"])]
  #[test_case("some #tag\nwith spaces", vec!["tag"])]
  #[test_case("two tags #tag and #Tag", vec!["tag", "Tag"])]
  #[test_case("invalid tags # end", vec![])]
  #[test_case("invalid tags #aaaa#bbbb end", vec![])]
  #[test_case("japanese space #　", vec![])]
  #[test_case("some #tags with\n#newlines", vec!["tags", "newlines"])]
  fn test_extract_tags(text: &str, result: Vec<&str>) {
    let result_vector: Vec<String> = result.iter().map(|s| (*s).to_owned()).collect();
    assert_eq!(extract_tags_non_unique(text), result_vector);
  }
}
