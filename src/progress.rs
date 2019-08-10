use indicatif::{FormattedDuration, ProgressBar, ProgressDrawTarget};
use std::time::Instant;

pub struct Progress {
	bar: ProgressBar,
	begin: Instant,
}

impl Progress {
	pub fn new(limit: u64) -> Progress {
		Progress {
			bar: ProgressBar::with_draw_target(limit, ProgressDrawTarget::stderr()),
			begin: Instant::now(),
		}
	}

	pub fn inc(&self) {
		self.bar.inc(1);
	}

	pub fn finish(&self) {
		self.bar.finish();
		eprintln!(
			"Calculation took {}",
			FormattedDuration(self.begin.elapsed())
		);
	}
}
