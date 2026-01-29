use releasor::progress::{animate_to, finish, progress_line, show, wait_animate};

#[test]
fn progress_line_zero_percent() {
    let line = progress_line(0, "starting");
    assert_eq!(line, format!("{} %0 starting", "-".repeat(40)));
}

#[test]
fn progress_line_hundred_percent() {
    let line = progress_line(100, "done");
    assert_eq!(line, format!("{} %100 done", "=".repeat(40)));
}

#[test]
fn progress_line_fifty_percent() {
    let line = progress_line(50, "half");
    let expected_bar = "=".repeat(20) + &"-".repeat(20);
    assert_eq!(line, format!("{} %50 half", expected_bar));
}

#[test]
fn progress_line_one_percent() {
    let line = progress_line(1, "step");
    assert_eq!(line, "---------------------------------------- %1 step");
}

#[test]
fn progress_line_empty_step() {
    let line = progress_line(0, "");
    assert_eq!(line, format!("{} %0 ", "-".repeat(40)));
}

#[test]
fn progress_line_bar_width_is_40() {
    let line = progress_line(25, "x");
    let bar: String = line
        .chars()
        .take_while(|c| *c == '=' || *c == '-')
        .collect();
    assert_eq!(bar.len(), 40, "bar should be 40 chars");
}

#[test]
fn show_does_not_panic() {
    show(0, "zero");
    show(50, "half");
    show(100, "full");
}

#[test]
fn animate_to_and_wait_completes() {
    finish();
    let handle = animate_to(5, "animate");
    wait_animate(handle);
}

#[test]
fn animate_to_zero_completes_immediately() {
    finish();
    let handle = animate_to(0, "zero");
    wait_animate(handle);
}

#[test]
fn finish_does_not_panic() {
    finish();
}

#[test]
fn finish_resets_state_for_next_animation() {
    finish();
    let handle = animate_to(2, "first");
    wait_animate(handle);
    finish();

    let handle2 = animate_to(1, "second");
    wait_animate(handle2);
    finish();
}
