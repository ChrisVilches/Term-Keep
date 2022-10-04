use crate::models::traits::FromSqlRow;

#[derive(Debug, Clone)]
pub struct Template {
  pub id: Option<u32>,
  pub name: String,
  pub content: String,
}

impl FromSqlRow for Template {
  fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
    Ok(Template {
      id: row.get(0)?,
      name: row.get(1)?,
      content: row.get(2)?,
    })
  }
}

impl Default for Template {
  fn default() -> Self {
    Template {
      id: None,
      name: String::new(),
      content: String::new(),
    }
  }
}
