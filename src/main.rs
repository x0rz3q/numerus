#[macro_use]
extern crate clap;
mod progress;
mod sequences;

use clap::{App, Arg, SubCommand};
use std::process::exit;

use sequences::{
	fibonacci::Fibonacci,
	pentagonal::Pengagonal,
	power::{Cube, Square},
	triangular,
	tribonacci::Tribonacci,
	Sequence,
};

fn main() {
	let mut app = App::new("numerus")
		.version(crate_version!())
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
		.subcommand(SubCommand::with_name("cube").about("Calculate the cube sequence"))
		.subcommand(SubCommand::with_name("triangular").about("Calculate the triangular sequence"))
		.subcommand(SubCommand::with_name("pentagonal").about("Calculate the pentagonal sequence"));
	let matches = app.clone().get_matches();

	let limit = match matches.value_of("limit").unwrap_or("30").parse::<u64>() {
		Ok(limit) => limit,
		Err(_) => {
			println!("Limit should be an integer");
			exit(1);
		},
	};

	match matches.subcommand_name() {
		Some("fibonacci") => Fibonacci::calculate(10),
		Some("tribonacci") => Tribonacci::calculate(limit),
		Some("square") => Square::calculate(limit),
		Some("cube") => Cube::calculate(limit),
		Some("triangular") => triangular::calculate(limit),
		Some("pentagonal") => Pengagonal::calculate(limit),
		_ => {
			app.print_help().unwrap();
			println!("");
		},
	};
}
