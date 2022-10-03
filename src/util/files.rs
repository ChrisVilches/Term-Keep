use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

// TODO: Probably can be memoized.
pub fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>, std::io::Error> {
  let file = File::open(filename)?;
  let buf = BufReader::new(file);
  Ok(
    buf
      .lines()
      .map(|l| l.expect("Could not parse line"))
      .collect(),
  )
}
