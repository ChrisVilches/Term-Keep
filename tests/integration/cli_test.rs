use crate::runner::{exec_test, run_and_grep_stdout, run_app, run_error, run_success};
use crate::util::randomize_cases;
use term_keep::models::note::Note;
use term_keep::models::note_type::NoteType;
use term_keep::models::task_status::TaskStatus;
use term_keep::services;

#[test]
fn test_info() {
  exec_test(|| {
    services::notes::create_note("some text").unwrap();

    let (stdout, _, exit_status) = run_app(&["info"]);

    assert!(stdout.contains("Database location"));
    assert!(stdout.contains("Can read tips"));
    assert!(exit_status.success());
  });
}

#[test]
fn test_note_count() {
  exec_test(|| {
    assert!(run_success(&[]).contains("0 note(s)"));
  });

  exec_test(|| {
    services::notes::create_note("some text").unwrap();
    assert!(run_success(&[]).contains("1 note(s)"));
  });
}

#[test]
fn test_subtasks() {
  exec_test(|| {
    services::notes::create_task("- [] task\n- [x] completed").unwrap();
    assert!(run_success(&[]).contains("(1 / 2)"));
  });

  exec_test(|| {
    services::notes::create_task("- [x] task\n- [x] completed").unwrap();
    assert!(run_success(&[]).contains("(2 / 2)"));
  });
}

#[test]
fn test_remove() {
  exec_test(|| {
    services::notes::create_task("new note").unwrap();
    let stderr = run_error(&["rm", "1"]);
    assert_eq!(services::notes::find_all(false).len(), 1);
    assert_eq!(
      stderr,
      "Error: The note must be archived before removing permanently\n"
    );
  });

  exec_test(|| {
    services::notes::create_task("new note").unwrap();
    run_app(&["archive", "1"]);
    let stdout = run_success(&["rm", "1"]);
    assert!(stdout.contains("Removed"));
    assert!(services::notes::find_all(false).is_empty());
  });
}

#[test]
fn test_show() {
  exec_test(|| {
    assert_eq!(
      run_error(&["show", "1"]),
      "Error: note not found with ID = 1\n"
    );
  });

  exec_test(|| {
    services::notes::create_note("some text").unwrap();
    let (stdout, _, exit_status) = run_app(&["show", "1"]);
    assert!(stdout.contains("some text"));
    assert!(!stdout.contains("(Updated: "));
    assert!(exit_status.success());
  });

  exec_test(|| {
    services::notes::create_note("some text").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(1100));
    services::notes::update(1, "modified text").unwrap();
    let (stdout, _, exit_status) = run_app(&["show", "1"]);
    assert!(stdout.contains("(Updated: "));
    assert!(stdout.contains("modified text"));
    assert!(exit_status.success());
  });
}

#[test]
fn test_pin() {
  let get_pinned_notes = || -> Vec<Note> {
    services::notes::find_all(false)
      .into_iter()
      .filter(|n| n.pinned)
      .collect()
  };

  exec_test(|| {
    services::notes::create_note("some pinned note").unwrap();
    assert!(get_pinned_notes().is_empty());
    run_app(&["pin", "1"]);
    assert_eq!(get_pinned_notes().len(), 1);
    run_app(&["pin", "1", "-r"]);
    assert!(get_pinned_notes().is_empty());
  });

  exec_test(|| {
    services::notes::create_note("some pinned note").unwrap();
    run_app(&["pin", "1"]);
    let (stdout, _, exit_status) = run_app(&["pin", "1"]);
    assert_eq!(stdout, "Not changed\n");
    assert!(exit_status.success());
  });
}

#[test]
fn test_pin_display_order() {
  exec_test(|| {
    services::notes::create_note("test note 1").unwrap();
    services::notes::create_note("test note 2").unwrap();
    services::notes::create_note("test note 3").unwrap();
    services::notes::create_note("test note 4").unwrap();
    run_app(&["pin", "3"]);

    let note_summaries = run_and_grep_stdout(&[], "test note ");

    assert_eq!(note_summaries.len(), 4);
    assert!(note_summaries[0].contains("test note 3"));
    assert!(note_summaries[0].contains("📌"));
    assert!(note_summaries[1].contains("test note 1"));
    assert!(note_summaries[2].contains("test note 2"));
    assert!(note_summaries[3].contains("test note 4"));
  });
}

#[test]
fn test_task_status() {
  exec_test(|| {
    services::notes::create_task("my task").unwrap();

    run_app(&["change-task-status", "1", &randomize_cases("done")]);
    assert_eq!(
      services::notes::find_one(1).unwrap().note_type,
      NoteType::Task(TaskStatus::Done)
    );

    run_app(&["change-task-status", "1", &randomize_cases("todo")]);
    assert_eq!(
      services::notes::find_one(1).unwrap().note_type,
      NoteType::Task(TaskStatus::Todo)
    );

    run_app(&["change-task-status", "1", &randomize_cases("progress")]);
    assert_eq!(
      services::notes::find_one(1).unwrap().note_type,
      NoteType::Task(TaskStatus::Progress)
    );
  });

  exec_test(|| {
    services::notes::create_task("my task").unwrap();
    assert_eq!(
      run_error(&["change-task-status", "1", "aaaaaaa"]),
      "Error: Invalid status (allowed values: todo, progress, done)\n"
    )
  });

  exec_test(|| {
    services::notes::create_note("my note").unwrap();
    assert_eq!(
      run_error(&["change-task-status", "1", "aaaaaaa"]),
      "Error: Not a task\n"
    )
  });
}

#[test]
fn test_show_tags() {
  exec_test(|| {
    services::notes::create_task("my task #some #tag").unwrap();
    services::notes::create_note("my note #some #another-tag").unwrap();
    let (stdout, _, exit_status) = run_app(&["tags"]);
    let lines: Vec<&str> = stdout.split('\n').collect();
    assert!(exit_status.success());
    assert!(lines[0].contains("(2 notes) #some"));
    // TODO: Should be "1 note"
    assert!(lines[1].contains("(1 notes) #another-tag"));
  });
}

#[test]
fn test_archive_all_done() {
  let setup = || {
    services::notes::create_task("test task 1").unwrap();
    services::notes::create_task("test task 2").unwrap();
    services::notes::create_task("test task 3").unwrap();
    services::notes::create_task("test task 4").unwrap();
    services::notes::create_task("test task 5").unwrap();
    services::notes::create_task("test task 6").unwrap();

    run_app(&["change-task-status", "1", "done"]);
    run_app(&["change-task-status", "2", "progress"]);
    run_app(&["change-task-status", "3", "done"]);
    run_app(&["change-task-status", "4", "todo"]);
    run_app(&["change-task-status", "5", "progress"]);
    run_app(&["change-task-status", "6", "done"]);
  };

  exec_test(|| {
    setup();
    let note_summaries = run_and_grep_stdout(&[], "test task ");

    assert_eq!(note_summaries.len(), 6);
    assert!(note_summaries[0].contains("[ ✔ ] test task 1"));
    assert!(note_summaries[1].contains("[ - ] test task 2"));
    assert!(note_summaries[2].contains("[ ✔ ] test task 3"));
    assert!(note_summaries[3].contains("[   ] test task 4"));
    assert!(note_summaries[4].contains("[ - ] test task 5"));
    assert!(note_summaries[5].contains("[ ✔ ] test task 6"));
  });

  exec_test(|| {
    setup();
    run_app(&["archive-all-done"]);
    let note_summaries = run_and_grep_stdout(&[], "test task ");

    assert_eq!(note_summaries.len(), 3);
    assert!(note_summaries[0].contains("[ - ] test task 2"));
    assert!(note_summaries[1].contains("[   ] test task 4"));
    assert!(note_summaries[2].contains("[ - ] test task 5"));
    assert_eq!(services::notes::find_all(true).len(), 3);
  });
}

#[test]
fn test_search_fuzzy() {
  let setup = || {
    services::notes::create_note("hello heelloo hello hello").unwrap();
    services::notes::create_note("hello world").unwrap();
    services::notes::create_note("byebye world").unwrap();
  };

  exec_test(|| {
    setup();
    let search_results = run_and_grep_stdout(&["search", "hello"], "(score ");

    assert_eq!(search_results.len(), 2);
    assert!(search_results[0].contains("hello heelloo hello hello"));
    assert!(search_results[1].contains("hello world"));
  });

  exec_test(|| {
    setup();
    let search_results = run_and_grep_stdout(&["search", "bye"], "(score ");

    assert_eq!(search_results.len(), 1);
    assert!(search_results[0].contains("byebye world"));
  });

  exec_test(|| {
    setup();
    assert_eq!(
      run_success(&["search", "qwerty123456"]),
      "0 results for qwerty123456\n"
    );
  });
}

#[test]
fn test_search_by_tags() {
  let setup = || {
    services::notes::create_note("#rust #javascript test-note").unwrap();
    services::notes::create_note("#RuST #note test-note").unwrap();
    services::notes::create_note("coding some #JavaScript test-note").unwrap();
  };

  exec_test(|| {
    setup();
    let rust = run_and_grep_stdout(&["search", "rust", "-t"], "test-note");
    assert_eq!(rust.len(), 2);

    let js = run_and_grep_stdout(&["search", "Javascript", "-t"], "test-note");
    assert_eq!(js.len(), 2);

    let note_tag = run_and_grep_stdout(&["search", "notE", "-t"], "test-note");
    assert_eq!(note_tag.len(), 1);

    let not_found = run_and_grep_stdout(&["search", "javascripts", "-t"], "test-note");
    assert_eq!(not_found.len(), 0);
  });
}

#[test]
fn test_templates() {
  // TODO: Implement
  println!("Not implemented yet!");
  // todo!();
}
