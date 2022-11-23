use crate::models::traits::FromSqlRow;
use crate::models::traits::ModelName;

use super::traits::RequireId;

#[derive(Debug, Clone, Default)]
pub struct Template {
  pub id: Option<u32>,
  pub name: String,
  pub content: String,
}

impl ModelName for Template {
  fn model_name() -> String {
    "template".to_owned()
  }
}

impl FromSqlRow for Template {
  fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
    Ok(Self {
      id: row.get("id")?,
      name: row.get("name")?,
      content: row.get("content")?,
    })
  }
}

impl RequireId for Template {
  fn option_id(&self) -> Option<u32> {
    self.id
  }
}
