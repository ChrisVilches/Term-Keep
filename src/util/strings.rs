pub fn truncate_string_ellipsis(s: String, length: usize) -> String {
  let mut new_s = s;

  if new_s.len() <= length {
    new_s
  } else {
    new_s.truncate(length);
    new_s + "..."
  }
}

pub fn bool_to_str(b: bool) -> String {
  match b {
    true => "Yes".to_string(),
    false => "No".to_string(),
  }
}

pub fn first_line(s: String) -> String {
  s.split('\n')
    .filter(|s| !s.trim().is_empty())
    .collect::<Vec<&str>>()
    .first()
    .unwrap_or(&"")
    .to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_first_line() {
    assert_eq!(first_line("hello\nworld".to_string()), "hello");
    assert_eq!(first_line("  a \n b".to_string()), "  a ");
    assert_eq!(first_line(" only one line ".to_string()), " only one line ");
    assert_eq!(
      first_line("  \n first line is blank ".to_string()),
      " first line is blank "
    );
    assert_eq!(
      first_line("\n first line is blank ".to_string()),
      " first line is blank "
    );
    assert_eq!(first_line("\n \n   ".to_string()), "");
  }

  #[test]
  fn test_truncate_string_ellipsis() {
    assert_eq!(truncate_string_ellipsis("abcde".to_string(), 2), "ab...");
    assert_eq!(truncate_string_ellipsis("abcde".to_string(), 7), "abcde");
  }

  #[test]
  fn test_bool_to_str() {
    assert_eq!(bool_to_str(true), "Yes");
    assert_eq!(bool_to_str(false), "No");
  }
}
