use std::io::{self, Write};
use std::sync::atomic::{AtomicU8, Ordering};
use std::thread;
use std::time::Duration;

const BAR_WIDTH: usize = 40;
const ANIMATE_MS: u64 = 35;

static CURRENT_PCT: AtomicU8 = AtomicU8::new(0);

/// Renders a text progress bar and updates the current line.
pub fn show(percent: u8, step: &str) {
    let filled = (percent as usize * BAR_WIDTH) / 100;
    let empty = BAR_WIDTH.saturating_sub(filled);
    let bar: String = "=".repeat(filled) + &"-".repeat(empty);
    let line = format!("{} %{} {}", bar, percent, step);
    print!("\r{:<80}", line);
    io::stdout().flush().unwrap();
}

/// Starts a thread that animates from current % to target % (e.g. 50, 51, 52…).
/// Run your work, then call `wait_animate(handle)`.
pub fn animate_to(target: u8, step: &str) -> thread::JoinHandle<()> {
    let step = step.to_string();
    thread::spawn(move || {
        loop {
            let current = CURRENT_PCT.load(Ordering::Relaxed);

            if current >= target {
                break;
            }

            let next = current.saturating_add(1).min(target);
            CURRENT_PCT.store(next, Ordering::Relaxed);
            show(next, &step);
            thread::sleep(Duration::from_millis(ANIMATE_MS));
        }
    })
}

/// Waits for the animation thread to reach its target.
pub fn wait_animate(handle: thread::JoinHandle<()>) {
    let _ = handle.join();
}

/// Finishes the progress bar (newline so subsequent output is on a new line).
/// Resets the internal percentage so the progress bar can be used again in the same process.
pub fn finish() {
    println!();
    CURRENT_PCT.store(0, Ordering::Relaxed);
}
