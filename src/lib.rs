pub mod composer;
pub mod instrument;
pub mod file_export;
pub mod prelude;

use indicatif::ProgressStyle;

fn default_progress_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "  {msg}:\n{bar} {pos:>3}/{len:3}"
    ).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }
}
