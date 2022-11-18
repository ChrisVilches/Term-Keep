use colored::Colorize;

fn format_search_results_header(query: &str, count: usize) -> String {
  format!("{} results for {}", count.to_string().bold(), query.bold())
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
