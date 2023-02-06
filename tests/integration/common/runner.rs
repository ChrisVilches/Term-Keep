use std::fs;
use std::{
  process::{Command, ExitStatus},
  sync::{Mutex, Once},
};
use term_keep::services;

static TEST_MUTEX: Mutex<()> = Mutex::new(());
static COMPILE_APP: Once = Once::new();

const TMP_DB_FILE: &str = "/tmp/term_keep_test.db";
const EXECUTABLE_PATH: &str = "./target/debug/term_keep";
// TODO: Rename this? The purpose of this function is to provide a wrapper
//       for a critical section (exclusive execution) and a clean database.
pub fn exec_test(callback: impl Fn()) {
  let guard = TEST_MUTEX
    .lock()
    .expect("Should be able to obtain test lock");
  fs::remove_file(TMP_DB_FILE).expect("Should be able to remove tmp file");
  services::db::set_database(TMP_DB_FILE).expect("Should be able to install test database");
  callback();
  std::mem::drop(guard);
}

fn compile() {
  COMPILE_APP.call_once(|| {
    let success = Command::new("cargo")
      .args(["build"])
      .status()
      .expect("Should compile app correctly")
      .success();

    assert!(success);
  });
}

// TODO: Marked as "never used", most likely because it's only used by the test environment, so Rust compiler
//       is trying to detect the "debug" or "release" environment. Maybe????
pub fn run_app(args: &[&str]) -> (String, String, ExitStatus) {
  compile();

  let output = Command::new(EXECUTABLE_PATH)
    .args(args)
    .env("TERM_KEEP_DB_PATH", TMP_DB_FILE)
    .output()
    .expect("Should be able to run command");

  let stdout = String::from_utf8(output.stdout).unwrap();
  let stderr = String::from_utf8(output.stderr).unwrap();
  let exit_code = output.status;

  (stdout, stderr, exit_code)
}

pub fn run_success(args: &[&str]) -> String {
  let (stdout, _, exit_status) = run_app(args);
  assert!(exit_status.success());
  return stdout;
}

pub fn run_error(args: &[&str]) -> String {
  let (stdout, stderr, exit_status) = run_app(args);
  assert!(stdout.is_empty());
  assert_eq!(exit_status.code(), Some(1));
  return stderr;
}

pub fn run_and_grep_stdout(args: &[&str], pattern: &str) -> Vec<String> {
  run_app(args)
    .0
    .split('\n')
    .filter(|s| s.contains(pattern))
    .map(str::to_owned)
    .collect()
}
