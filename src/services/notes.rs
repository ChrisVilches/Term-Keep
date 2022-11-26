use crate::errors::not_found_by_id_error::NotFoundByIdError;
use crate::errors::row_not_changed_error::RowNotChangedError;
use crate::models::note::Note;
use crate::models::task_status::TaskStatus;
use crate::services::db::change_row;
use crate::services::db::change_rows;
use crate::services::db::rows_to_vec;
use crate::services::db::single_row;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use rayon::prelude::*;
use std::cmp::Ordering;

pub fn find_all(only_archived: bool) -> Vec<Note> {
  rows_to_vec(
    "SELECT id, content, pinned, archived, task_status, created_at, updated_at FROM note WHERE archived = ?",
    rusqlite::params![only_archived],
  )
}

pub fn find_all_include_archived() -> Vec<Note> {
  rows_to_vec(
    "SELECT id, content, pinned, archived, task_status, created_at, updated_at FROM note",
    rusqlite::params![],
  )
}

pub fn find_one(id: u32) -> Result<Note, NotFoundByIdError> {
  single_row::<Note>(
    "SELECT id, content, pinned, archived, task_status, created_at, updated_at FROM note WHERE id = ? LIMIT 1",
    rusqlite::params![id],
  )
  .ok_or_else(|| NotFoundByIdError::new::<Note>(id))
}

pub fn find_latest() -> Option<Note> {
  single_row::<Note>(
    "SELECT id, content, pinned, archived, task_status, created_at, updated_at FROM note ORDER BY id DESC LIMIT 1",
    rusqlite::params![],
  )
}

fn cmp((score1, n1): &(i64, Note), (score2, _): &(i64, Note)) -> Ordering {
  let ord = score2.cmp(score1);

  if ord == Ordering::Equal {
    if n1.pinned {
      Ordering::Less
    } else {
      Ordering::Greater
    }
  } else {
    ord
  }
}

pub fn fuzzy_search(text: &str, archived: bool) -> Vec<(i64, Note)> {
  let notes: Vec<Note> = find_all(archived);

  let matcher = SkimMatcherV2::default();

  let mut results: Vec<(i64, Note)> = notes
    .into_par_iter()
    .map(|note| (matcher.fuzzy_match(&note.content, text).unwrap_or(0), note))
    .filter(|pair| pair.0 > 0)
    .collect();

  results.par_sort_unstable_by(cmp);
  results
}

pub fn create_note(text: &str) -> Result<(), RowNotChangedError> {
  change_row::<Note>(
    "INSERT INTO note (content) VALUES (?)",
    rusqlite::params![text],
  )
}

pub fn create_task(text: &str) -> Result<(), RowNotChangedError> {
  change_row::<Note>(
    "INSERT INTO note (content, task_status) VALUES (?, 0)",
    rusqlite::params![text],
  )
}

pub fn update(id: u32, text: &str) -> Result<(), RowNotChangedError> {
  change_row::<Note>(
    "UPDATE note SET content = ? WHERE id = ?",
    rusqlite::params![text, id],
  )
}

pub fn pin(id: u32, pinned: bool) -> Result<(), RowNotChangedError> {
  change_row::<Note>(
    "UPDATE note SET pinned = ? WHERE id = ?",
    rusqlite::params![pinned, id],
  )
}

pub fn archive(id: u32, archived: bool) -> Result<(), RowNotChangedError> {
  change_row::<Note>(
    "UPDATE note SET archived = ? WHERE id = ?",
    rusqlite::params![archived, id],
  )
}

pub fn change_task_status(id: u32, status: i32) -> Result<(), RowNotChangedError> {
  change_row::<Note>(
    "UPDATE note SET task_status = ? WHERE id = ?",
    rusqlite::params![status, id],
  )
}

pub fn archive_all_done() -> usize {
  change_rows::<Note>(
    "UPDATE note SET archived = true WHERE task_status = ? AND archived = false",
    rusqlite::params![TaskStatus::Done],
  )
}

pub fn remove(id: u32) -> Result<(), RowNotChangedError> {
  change_row::<Note>(
    "DELETE FROM note WHERE id = ? AND archived = ?",
    rusqlite::params![id, true],
  )
}
