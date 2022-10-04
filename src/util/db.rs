use crate::models::traits::FromSqlRow;

pub fn row_to_template<T: FromSqlRow>(row: &rusqlite::Row) -> Result<T, rusqlite::Error> {
  T::from_row(row)
}

pub fn rows_to_vec<T: FromSqlRow>(mut stmt: rusqlite::Statement, params: &[&dyn rusqlite::ToSql]) -> Vec<T> {
  stmt
    .query_map(params, row_to_template::<T>)
    .unwrap()
    .map(|n| n.unwrap())
    .collect()
}
