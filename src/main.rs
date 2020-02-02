#[macro_use]
extern crate clap;
mod sequences;

use clap::{App, Arg, SubCommand};
use std::process::exit;

use sequences::{fibonacci, pentagonal, power, triangular, tribonacci};

fn main() {
	let mut power_command = SubCommand::with_name("power")
		.about("Calculate the power sequence")
		.arg(
			Arg::with_name("power")
				.help("The power to be used")
				.index(1),
		);
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
		.subcommand(power_command.clone())
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
		Some("fibonacci") => fibonacci::calculate(limit),
		Some("tribonacci") => tribonacci::calculate(limit),
		Some("square") => power::calculate(limit, 2),
		Some("cube") => power::calculate(limit, 3),
		Some("triangular") => triangular::calculate(limit),
		Some("pentagonal") => pentagonal::calculate(limit),
		Some("power") => {
			if let Some(matches) = matches.subcommand_matches("power") {
				let power = match matches.value_of("power") {
					Some(power) => power,
					None => {
						power_command.print_help().unwrap();
						println!("");
						exit(1);
					},
				};

				let power = match power.parse::<u64>() {
					Ok(power) => power,
					Err(_) => {
						println!("Power needs to be an integer");
						exit(1);
					},
				};

				power::calculate(limit, power);
			}
		},
		_ => {
			app.print_help().unwrap();
			println!("");
		},
	};
}
