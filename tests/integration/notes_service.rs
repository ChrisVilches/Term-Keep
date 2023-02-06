use crate::common::runner::exec_test;
use term_keep::services;

#[test]
fn test_empty_notes() {
  exec_test(|| {
    std::thread::sleep(std::time::Duration::from_millis(200));
    let notes = services::notes::find_all(false);
    assert!(notes.is_empty());
  });
}

#[test]
fn test_add_one_note() {
  exec_test(|| {
    services::notes::create_note("some text").unwrap();
    let notes = services::notes::find_all(false);
    assert_eq!(notes.len(), 1);
    assert_eq!(
      notes.first().cloned().map(|n| n.content).unwrap(),
      "some text"
    );
  });
}

#[test]
fn test_archived() {
  exec_test(|| {
    services::notes::create_note("some text 1").unwrap();
    services::notes::create_note("some text 2").unwrap();
    services::notes::create_task("some text 3").unwrap();
    services::notes::create_task("some text 4").unwrap();

    assert_eq!(services::notes::find_all(false).len(), 4);

    services::notes::archive(1, true).unwrap();

    let notes = services::notes::find_all(false);
    assert_eq!(notes.len(), 3);
    assert_eq!(notes[0].content, "some text 2");
    assert_eq!(notes[1].content, "some text 3");
    assert_eq!(notes[2].content, "some text 4");
    assert_eq!(services::notes::find_all_include_archived().len(), 4);
  });
}
