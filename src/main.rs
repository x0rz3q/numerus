extern crate clap;
use clap::{App, Arg, SubCommand};
use std::process::exit;

/* number sequences */
mod fibonacci;
mod square;
mod triangular;
mod tribonacci;

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
				.help("Calculation limit"),
		)
		.subcommand(SubCommand::with_name("fibonacci").about("Calculate the fibonacci sequence"))
		.subcommand(SubCommand::with_name("tribonacci").about("Calculate the tribonacci sequence"))
		.subcommand(SubCommand::with_name("square").about("Calculate the square sequence"))
		.subcommand(SubCommand::with_name("triangular").about("Calculate the triangular sequence"))
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
		Some("tribonacci") => tribonacci::calculate(limit),
		Some("square") => square::calculate(limit),
		Some("triangular") => triangular::calculate(limit),
		_ => {
			println!("Please specifiy a sequence");
			exit(1);
		}
	};
}
