use indicatif::{ProgressBar, ProgressDrawTarget};

pub fn get_bar(limit: u64) -> ProgressBar {
	let bar = ProgressBar::with_draw_target(limit, ProgressDrawTarget::stderr());
	bar.println(format!("Generating {} numbers", limit));

	bar
}
