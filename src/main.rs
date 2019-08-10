extern crate clap;
use clap::{Arg, App, SubCommand};
use std::process::exit;

/* number sequences */
mod fibonacci;

fn main() {
	let matches = App::new("numerus")
		.version("0.1.0")
		.author("J.M. Thiessen <jacob@x0rz3q.com>")
		.about("Calculate number sequences")
		.arg(
			Arg::with_name("limit")
				.short("l")
				.long("limit")
				.takes_value(true)
				.index(1)
				.help("Calculation limit")
		)
		.subcommand(SubCommand::with_name("fibonacci")
					.about("Calculate the fibonacci sequence")
					)
		.get_matches();

	let limit = match matches.value_of("limit").unwrap_or("30").parse::<u64>() {
		Ok(limit) => limit,
		Err(_) => {
			println!("Limit should be an integer");
			exit(1);
		}
	};

	match matches.subcommand_name() {
		Some("fibonacci") => fibonacci::calculate(limit),
		_ => {
			println!("Please specifiy a sequence");
			exit(1);
		}
	};
}