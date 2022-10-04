pub fn abort_with_message(msg: String) {
  eprintln!("{}", msg);
  std::process::exit(1);
}
