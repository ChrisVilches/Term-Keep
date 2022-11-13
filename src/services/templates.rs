use crate::errors::not_found_by_field_error::NotFoundByFieldError;
use crate::errors::row_not_changed_error::RowNotChangedError;
use crate::models::template::Template;
use crate::services::db::change_row;
use crate::services::db::rows_to_vec;
use crate::services::db::single_row;

pub fn find_all() -> Vec<Template> {
  rows_to_vec(
    "SELECT id, name, content FROM template",
    rusqlite::params![],
  )
}

pub fn find_one(name: &String) -> Result<Template, NotFoundByFieldError> {
  single_row::<Template>(
    "SELECT id, name, content FROM template WHERE name = ?",
    rusqlite::params![name],
  )
  .ok_or_else(|| NotFoundByFieldError::new::<Template>("name".to_owned(), name.to_string()))
}

pub fn create(name: &String, content: &String) -> Result<(), RowNotChangedError> {
  change_row::<Template>(
    "INSERT INTO template (name, content) VALUES (?, ?)",
    rusqlite::params![name, content],
  )
}

pub fn update(id: u32, content: &String) -> Result<(), RowNotChangedError> {
  change_row::<Template>(
    "UPDATE template SET content = ? WHERE id = ?",
    rusqlite::params![content, id],
  )
}

pub fn remove(id: u32) -> Result<(), RowNotChangedError> {
  change_row::<Template>("DELETE FROM template WHERE id = ?", rusqlite::params![id])
}
