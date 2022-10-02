pub fn truncate_string_ellipsis(s: String, length: usize) -> String {
  let mut new_s = String::from(s);

  if new_s.len() <= length {
    return new_s;
  }

  new_s.truncate(length);
  new_s + "..."
}
