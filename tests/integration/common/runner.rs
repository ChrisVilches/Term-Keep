use std::fs;
use std::io::Write;
use std::process::Stdio;
use std::{
  process::{Command, ExitStatus},
  sync::{Mutex, Once},
};
use term_keep::services;

static TEST_MUTEX: Mutex<()> = Mutex::new(());
static COMPILE_APP: Once = Once::new();

const TMP_DB_FILE: &str = "/tmp/term_keep_test.db";
const EXECUTABLE_PATH: &str = "./target/debug/term_keep";

pub fn exec_test(callback: impl Fn()) {
  /*
   * TODO: If one test fails, the mutex will become poisoned, but the idea
   * is that other tests can continue executing. So try to clear poison from the
   * mutex. This functionality cannot be used because it's unstable, so try later.
   * Current issue: if one test fails, the output becomes hard to understand because
   * many tests will fail (because they can't acquire the lock).
   * Note: This is only a problem while writing tests and debugging. If all tests are OK,
   * then this is not a problem, because the output will be clean.
   */
  let guard = TEST_MUTEX
    .lock()
    .expect("Should be able to obtain test lock");
  let _ = fs::remove_file(TMP_DB_FILE);
  services::db::set_database(TMP_DB_FILE).expect("Should be able to install test database");
  callback();

  std::mem::drop(guard);
}

fn ensure_compiled() {
  COMPILE_APP.call_once(|| {
    let success = Command::new("cargo")
      .args(["build"])
      .status()
      .expect("Should compile app correctly")
      .success();

    assert!(success);
  });
}

pub fn run_app(args: &[&str]) -> (String, String, ExitStatus) {
  run_app_with_stdin(args, None)
}

pub fn run_app_with_stdin(args: &[&str], stdin: Option<&str>) -> (String, String, ExitStatus) {
  ensure_compiled();

  let child = Command::new(EXECUTABLE_PATH)
    .args(args)
    .env("TERM_KEEP_DB_PATH", TMP_DB_FILE)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("Should create child process");

  if let Some(stdin_content) = stdin {
    child
      .stdin
      .as_ref()
      .unwrap()
      .write_all(stdin_content.as_bytes())
      .unwrap();
  }

  let output = child
    .wait_with_output()
    .expect("Command should end correctly");
  let stdout = String::from_utf8(output.stdout).unwrap();
  let stderr = String::from_utf8(output.stderr).unwrap();
  let exit_code = output.status;

  (stdout, stderr, exit_code)
}

pub fn run_success(args: &[&str]) -> String {
  let (stdout, _, exit_status) = run_app_with_stdin(args, None);
  assert!(exit_status.success());
  return stdout;
}

pub fn run_error(args: &[&str]) -> String {
  let (stdout, stderr, exit_status) = run_app_with_stdin(args, None);
  assert!(stdout.is_empty());
  assert_eq!(exit_status.code(), Some(1));
  return stderr;
}

pub fn run_and_grep_stdout(args: &[&str], pattern: &str) -> Vec<String> {
  run_success(args)
    .split('\n')
    .filter(|s| s.contains(pattern))
    .map(str::to_owned)
    .collect()
}
