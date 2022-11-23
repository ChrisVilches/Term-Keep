use colored::Colorize;

fn format_search_results_header(query: &str, count: usize) -> String {
  format!(
    "{} {} for {}",
    count.to_string().bold(),
    results_string(count),
    query.bold()
  )
}

const fn results_string(count: usize) -> &'static str {
  match count {
    1 => "result",
    _ => "results",
  }
}

pub fn format_search_results<T, F>(query: &str, notes: &Vec<T>, format_note: F) -> String
where
  F: Fn(&T) -> String,
{
  let mut result = format_search_results_header(query, notes.len());

  if !notes.is_empty() {
    result += "\n";
  }

  for note in notes {
    result += "\n";
    result += &format_note(note);
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case(0, "results")]
  #[test_case(1, "result")]
  #[test_case(2, "results")]
  #[test_case(3, "results")]
  #[test_case(4, "results")]
  fn test_results_string(count: usize, result: &str) {
    assert_eq!(results_string(count), result);
  }
}
