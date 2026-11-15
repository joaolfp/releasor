use std::io::{self, IsTerminal, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::thread;
use std::time::Duration;

const BAR_WIDTH: usize = 40;
const ANIMATE_MS: u64 = 35;

static CURRENT_PCT: AtomicU8 = AtomicU8::new(0);

/// Handle to a running progress animation. Pass it to `wait_animate`.
pub struct Animation {
    handle: Option<thread::JoinHandle<()>>,
    stop: Arc<AtomicBool>,
    target: u8,
    step: String,
}

/// True when stdout is a real terminal and we're not running in CI.
/// In CI (e.g. GitHub Actions) `\r` is rendered as a new line, so the animated bar
/// would print one line per percent.
pub fn is_interactive() -> bool {
    io::stdout().is_terminal() && std::env::var_os("CI").is_none()
}

/// Builds the progress line string (bar + percent + step). Used by `show` and by tests.
pub fn progress_line(percent: u8, step: &str) -> String {
    let filled = (percent as usize * BAR_WIDTH) / 100;
    let empty = BAR_WIDTH.saturating_sub(filled);
    let bar: String = "=".repeat(filled) + &"-".repeat(empty);
    format!("\x1b[34m{}\x1b[0m {}% {}", bar, percent, step)
}

/// Renders a text progress bar and updates the current line (interactive terminals only).
pub fn show(percent: u8, step: &str) {
    if !is_interactive() {
        return;
    }

    let line = progress_line(percent, step);
    print!("\r{:<89}", line);
    io::stdout().flush().unwrap();
}

/// Starts animating from current % to target % (e.g. 50, 51, 52…).
/// Run your work, then call `wait_animate(animation)`.
///
/// When not interactive (CI / piped output), prints a single plain line instead.
pub fn animate_to(target: u8, step: &str) -> Animation {
    let stop = Arc::new(AtomicBool::new(false));
    let step = step.to_string();

    if !is_interactive() {
        println!("▶ {step}...");
        let _ = io::stdout().flush();
        return Animation {
            handle: None,
            stop,
            target,
            step,
        };
    }

    let thread_stop = Arc::clone(&stop);
    let thread_step = step.clone();
    let handle = thread::spawn(move || {
        while !thread_stop.load(Ordering::Relaxed) {
            let current = CURRENT_PCT.load(Ordering::Relaxed);

            if current >= target {
                break;
            }

            let next = current.saturating_add(1).min(target);
            CURRENT_PCT.store(next, Ordering::Relaxed);
            show(next, &thread_step);
            thread::sleep(Duration::from_millis(ANIMATE_MS));
        }
    });

    Animation {
        handle: Some(handle),
        stop,
        target,
        step,
    }
}

/// Stops the animation as soon as the work is done and jumps straight to the target %.
pub fn wait_animate(animation: Animation) {
    animation.stop.store(true, Ordering::Relaxed);

    if let Some(handle) = animation.handle {
        let _ = handle.join();
    }

    let current = CURRENT_PCT.load(Ordering::Relaxed);
    if current < animation.target {
        CURRENT_PCT.store(animation.target, Ordering::Relaxed);
        show(animation.target, &animation.step);
    }
}

/// Finishes the progress bar (newline so subsequent output is on a new line).
/// Resets the internal percentage so the progress bar can be used again in the same process.
pub fn finish() {
    if is_interactive() {
        println!();
    }
    CURRENT_PCT.store(0, Ordering::Relaxed);
}
