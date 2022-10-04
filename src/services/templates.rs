use crate::util::db::rows_to_vec;
use crate::models::template::Template;
use crate::services::db;

// TODO: This file is a bit messy because each service works a bit differently from each other.

pub fn find_all_templates() -> Result<Vec<Template>, rusqlite::Error> {
  let conn = db::connection();
  let stmt = conn.prepare("SELECT id, name, content FROM template")?;
  Ok(rows_to_vec(stmt, rusqlite::params![]))
}

pub fn find_one_template(name: &String) -> Option<Template> {
  let conn = db::connection();
  let stmt = conn
    .prepare("SELECT id, name, content FROM template WHERE name = ?")
    .unwrap();

  rows_to_vec::<Template>(stmt, rusqlite::params![name])
    .first()
    .map(|t| t.clone())
}

pub fn create(name: &String, content: &String) -> Result<(), rusqlite::Error> {
  let conn = db::connection();
  let stmt = conn.prepare("INSERT INTO template (name, content) VALUES (?, ?)");
  stmt?.execute([name, content])?;
  Ok(())
}

pub fn update(id: u32, content: &String) -> Result<(), rusqlite::Error> {
  let conn = db::connection();
  let stmt = conn.prepare("UPDATE template SET content = ? WHERE id = ?");
  stmt?.execute(rusqlite::params![content, id])?;
  Ok(())
}

pub fn remove(id: u32) -> Result<usize, rusqlite::Error> {
  let conn = db::connection();
  let stmt = conn.prepare("DELETE FROM template WHERE id = ?");
  let rows_changed = stmt?.execute(rusqlite::params![id])?;
  Ok(rows_changed)
}
