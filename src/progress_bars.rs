use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
static mut MULTI_PROGRESS: Option<MultiProgress> = None;

pub fn default_progress_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "  {msg}:\n{bar} {pos:>3}/{len:3}"
    ).unwrap()
}

pub unsafe fn add_progress_bar(bar: ProgressBar) -> ProgressBar {
    if MULTI_PROGRESS.is_none() {
        MULTI_PROGRESS = Some(MultiProgress::new());
    }

    let multi = MULTI_PROGRESS.as_ref().unwrap();

    multi.add(bar)
}
