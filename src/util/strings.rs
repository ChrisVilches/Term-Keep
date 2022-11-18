pub fn truncate_string_ellipsis(s: &str, length: usize) -> String {
  let mut new_s = s.to_owned();

  if new_s.len() <= length {
    new_s
  } else {
    new_s.truncate(length);
    new_s + "..."
  }
}

pub fn count_lines(s: &str) -> usize {
  s.split('\n')
    .map(str::trim)
    .filter(|line| !line.is_empty())
    .count()
}

pub const fn bool_to_str<'a>(b: bool) -> &'a str {
  if b {
    "Yes"
  } else {
    "No"
  }
}

pub fn first_line(s: &str) -> &str {
  s.split('\n')
    .filter(|line| !line.trim().is_empty())
    .collect::<Vec<&str>>()
    .first()
    .unwrap_or(&"")
}

pub fn markdown_highlight(s: &str) -> String {
  if s.is_empty() {
    String::new()
  } else {
    format!("**`{}`**", s)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_count_lines() {
    assert_eq!(count_lines(""), 0);
    assert_eq!(count_lines("a"), 1);
    assert_eq!(count_lines(" a \n b "), 2);
    assert_eq!(count_lines(" a \n b \n "), 2);
    assert_eq!(count_lines(" a \n  \n b"), 2);
    assert_eq!(count_lines(" a \n  \n b \n"), 2);
    assert_eq!(count_lines(" a \n b \n c \n"), 3);
    assert_eq!(count_lines(" a \n b \n c \n d"), 4);
    assert_eq!(count_lines(" \n\n\n\nx\n\n\n"), 1);
  }

  #[test]
  fn test_first_line() {
    assert_eq!(first_line("hello\nworld"), "hello");
    assert_eq!(first_line("  a \n b"), "  a ");
    assert_eq!(first_line(" only one line "), " only one line ");
    assert_eq!(
      first_line("  \n first line is blank "),
      " first line is blank "
    );
    assert_eq!(
      first_line("\n first line is blank "),
      " first line is blank "
    );
    assert_eq!(first_line("\n \n   "), "");
  }

  #[test]
  fn test_truncate_string_ellipsis() {
    assert_eq!(truncate_string_ellipsis("abcde", 2), "ab...");
    assert_eq!(truncate_string_ellipsis("abcde", 7), "abcde");
  }

  #[test]
  fn test_bool_to_str() {
    assert_eq!(bool_to_str(true), "Yes");
    assert_eq!(bool_to_str(false), "No");
  }

  #[test]
  fn test_markdown_highlight() {
    assert_eq!(markdown_highlight("aaa"), "**`aaa`**");
    assert_eq!(markdown_highlight("   "), "**`   `**");
    assert_eq!(markdown_highlight(""), "");
  }
}
