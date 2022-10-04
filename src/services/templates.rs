use crate::models::template::Template;
use crate::services::db::change_rows;
use crate::services::db::rows_to_vec;
use crate::services::db::single_row;

pub fn find_all_templates() -> Result<Vec<Template>, rusqlite::Error> {
  rows_to_vec(
    "SELECT id, name, content FROM template",
    rusqlite::params![],
  )
}

pub fn find_one_template(name: &String) -> Option<Template> {
  single_row::<Template>(
    "SELECT id, name, content FROM template WHERE name = ?",
    rusqlite::params![name],
  )
}

pub fn create(name: &String, content: &String) -> Result<usize, rusqlite::Error> {
  change_rows(
    "INSERT INTO template (name, content) VALUES (?, ?)",
    rusqlite::params![name, content],
  )
}

pub fn update(id: u32, content: &String) -> Result<usize, rusqlite::Error> {
  change_rows(
    "UPDATE template SET content = ? WHERE id = ?",
    rusqlite::params![content, id],
  )
}

pub fn remove(id: u32) -> Result<usize, rusqlite::Error> {
  change_rows("DELETE FROM template WHERE id = ?", rusqlite::params![id])
}
