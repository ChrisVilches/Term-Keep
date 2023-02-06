use fancy_regex::{Captures, Regex};
use once_cell::sync::Lazy;
use std::collections::HashSet;

use super::strings::highlight;

static TAG_REGEX: Lazy<Regex> = Lazy::new(|| {
  // TODO: Keep adding more symbols.
  let invalid_tag_symbols = r"\.,#${}[]()+!#$%&'/@:;";
  let escaped = invalid_tag_symbols
    .chars()
    .map(|c| format!("\\{c}"))
    .collect::<String>();

  let regex_str = format!(r"(?<=\s|^)#([^[\s{escaped}]]+)(?=\s|$)");

  Regex::new(&regex_str).unwrap()
});

#[must_use]
pub fn format_text(s: &str) -> String {
  TAG_REGEX
    .replace_all(s, |c: &Captures| highlight(&format!("#{}", &c[1])))
    .into_owned()
}

#[must_use]
pub fn extract_tags_non_unique(text: &str) -> Vec<String> {
  TAG_REGEX
    .captures_iter(text)
    .map(|cap| cap.expect("Should parse tags")[1].to_owned())
    .collect()
}

#[must_use]
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
  #[test_case("#aa-bb", vec!["aa-bb"])]
  #[test_case("#aaa_bbb", vec!["aaa_bbb"])]
  #[test_case("#a #a  #a #a", vec!["a", "a", "a", "a"])]
  // TODO: I think this should extract "#period" because the user may type a period after a tag.
  //       The problem is that there are several kinds of periods (e.g. in Japanese)
  //       so it's not so easy.
  #[test_case("#first #period. #last #slash/slash", vec!["first", "last"])]
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
