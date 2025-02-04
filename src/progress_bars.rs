use std::sync::Mutex;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
static MULTI_PROGRESS: Mutex<Option<MultiProgress>> = Mutex::new(None);

pub fn default_progress_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "  {msg}:\n{bar} {pos:>3}/{len:3}"
    ).unwrap()
}

pub unsafe fn add_progress_bar(bar: ProgressBar) -> ProgressBar {
    let mut multi_progress = MULTI_PROGRESS.lock().unwrap();

    if multi_progress.is_none() {
        *multi_progress = Some(MultiProgress::new());
    }

    let multi = multi_progress.as_ref().unwrap();

    multi.add(bar)
}
