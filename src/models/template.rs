use crate::models::traits::FromSqlRow;
use crate::models::traits::ModelName;

#[derive(Debug, Clone, Default)]
pub struct Template {
  pub id: Option<u32>,
  pub name: String,
  pub content: String,
}

impl ModelName for Template {
  fn model_name() -> String {
    "template".to_string()
  }
}

impl FromSqlRow for Template {
  fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
    Ok(Self {
      id: row.get(0)?,
      name: row.get(1)?,
      content: row.get(2)?,
    })
  }
}
