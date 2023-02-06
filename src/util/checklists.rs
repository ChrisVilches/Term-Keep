use once_cell::sync::Lazy;

use colored::ColoredString;
use colored::Colorize;
use fancy_regex::Regex;

static CHECKBOX_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"(^\s*-\s+\[[xX\s]?\]\s+)(.*[^\s].*)").unwrap());
static CHECKED: Lazy<ColoredString> = Lazy::new(|| "[ ✔ ]".dimmed().bold());
static UNCHECKED: Lazy<ColoredString> = Lazy::new(|| "[   ]".dimmed().bold());

fn item_checked(item: &str) -> bool {
  item.contains('x') || item.contains('X')
}

pub fn checklist_completion(s: &str) -> (i32, i32) {
  let mut total = 0;
  let mut complete = 0;

  for line in s.lines() {
    let captures = CHECKBOX_REGEX
      .captures(line)
      .expect("Should attempt to parse");

    if let Some(c) = captures {
      total += 1;
      if item_checked(&c[1]) {
        complete += 1;
      }
    }
  }

  (complete, total)
}

fn format_line(line: &str) -> String {
  let captures = CHECKBOX_REGEX
    .captures(line)
    .expect("Should attempt to parse");

  captures.map_or_else(
    || line.to_owned(),
    |c| {
      if item_checked(&c[1]) {
        format!("{} {}", *CHECKED, &c[2].dimmed())
      } else {
        format!("{} {}", *UNCHECKED, &c[2])
      }
    },
  )
}

pub fn format_checklist(s: &str) -> String {
  s.lines()
    .map(format_line)
    .collect::<Vec<String>>()
    .join("\n")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_format_checklist() {
    let y = CHECKED.to_string();
    let n = UNCHECKED.to_string();

    assert_eq!(format_checklist(" "), " ");
    assert_eq!(format_checklist("- [] "), "- [] ");
    assert_eq!(
      format_checklist("- [] \n - [] a"),
      format!("- [] \n{} a", n)
    );
    assert_eq!(format_checklist(" normal text "), " normal text ");
    assert_eq!(
      format_checklist("not task\n - [x] hello"),
      format!("not task\n{} {}", y, "hello".dimmed())
    );
    assert_eq!(
      format_checklist(" - [x] hello"),
      format!("{} {}", y, "hello".dimmed())
    );
    assert_eq!(
      format_checklist(" - [x] hello world\n  - [] bye world"),
      format!("{} {}\n{} bye world", y, "hello world".dimmed(), n)
    );
    assert_eq!(
      format_checklist(" - [ ] hello world and more words\n  - [X] bye world test"),
      format!(
        "{} hello world and more words\n{} {}",
        n,
        y,
        "bye world test".dimmed()
      )
    );
  }

  #[test]
  fn test_checklist_completion() {
    assert_eq!(checklist_completion("- [x] aaa"), (1, 1));
    assert_eq!(checklist_completion("- [] aaa"), (0, 1));
    assert_eq!(
      checklist_completion(
        "
      - [] task 1
      -   [] task 2
      -  [x]   task 3
    "
      ),
      (1, 3)
    );
    assert_eq!(
      checklist_completion(
        "
      - [] task 1
      -   [] task 2
      -  [x]   task 3
      -[] incorrect task
    "
      ),
      (1, 3)
    );
  }

  #[test]
  fn test_checklist_completion_empty() {
    let empty_checkbox = (0, 1);
    assert_eq!(checklist_completion("- [] x"), empty_checkbox);
    assert_eq!(checklist_completion("   -  [] x"), empty_checkbox);
    assert_eq!(checklist_completion("-  []   x"), empty_checkbox);
    assert_eq!(checklist_completion(" - []  x   "), empty_checkbox);
    assert_eq!(checklist_completion("- [] x"), empty_checkbox);
    assert_eq!(checklist_completion("   -  [ ] x"), empty_checkbox);
    assert_eq!(checklist_completion("-  [ ]   x"), empty_checkbox);
    assert_eq!(checklist_completion(" - [ ]  x   "), empty_checkbox);
  }

  #[test]
  fn test_checklist_completion_checked() {
    let checked_checkbox = (1, 1);
    assert_eq!(checklist_completion(" - [x]  x   "), checked_checkbox);
    assert_eq!(checklist_completion(" - [X]  x   "), checked_checkbox);
    assert_eq!(checklist_completion(" -   [x]   x   "), checked_checkbox);
    assert_eq!(checklist_completion("- [X]  x"), checked_checkbox);
    assert_eq!(checklist_completion(" - [x]  x   "), checked_checkbox);
    assert_eq!(checklist_completion(" - [X]  x   "), checked_checkbox);
    assert_eq!(checklist_completion(" -   [x]   x   "), checked_checkbox);
    assert_eq!(checklist_completion("- [X]  x"), checked_checkbox);
  }

  #[test]
  fn test_checklist_completion_invalid() {
    let not_checkbox = (0, 0);
    assert_eq!(checklist_completion("- []x"), not_checkbox);
    assert_eq!(checklist_completion("- []"), not_checkbox);
    assert_eq!(checklist_completion("- [] "), not_checkbox);
    assert_eq!(checklist_completion("- []"), not_checkbox);
    assert_eq!(checklist_completion("- [x]  "), not_checkbox);
    assert_eq!(checklist_completion("- [X]  "), not_checkbox);
    assert_eq!(checklist_completion("-[] s"), not_checkbox);
    assert_eq!(checklist_completion("[] s"), not_checkbox);
    assert_eq!(checklist_completion(" - [  ]  x   "), not_checkbox);
  }
}
