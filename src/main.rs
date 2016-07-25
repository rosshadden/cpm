extern crate clap;
extern crate toml;

use std::fs::File;
use std::io::Read;
use std::process::Command;
use std::str;

use clap::{ Arg, App, SubCommand };

fn main() {
	let matches = App::new("cpm")
		// TODO: get these from `Cargo.toml`
		.version("0.0.0")
		.author("Ross Hadden <rosshadden@mail.com>")
		.subcommand(
			SubCommand::with_name("run")
				.about("Run actions specified in cosmic.toml.")
				.arg(
					Arg::with_name("action")
				)
		)
		.get_matches();

	if let Some(matches) = matches.subcommand_matches("run") {
		let action;
		if matches.is_present("action") {
			action = matches.value_of("action").unwrap();
		} else {
			action = "run";
		}

		let mut cosmicContents = String::new();

		File::open("cosmic.toml")
			.and_then(|mut file| { file.read_to_string(&mut cosmicContents) })
			.unwrap();

		let cosmic = toml::Parser::new(&cosmicContents).parse().unwrap();
		let ref actions = cosmic["actions"];
		let actionCmd = actions.lookup(action).unwrap().as_str().unwrap();

		let mut actionCmdIter = actionCmd.split(' ');
		let cmd = actionCmdIter.next().unwrap();
		let args: Vec<&str> = actionCmdIter.collect();

		Command::new(cmd)
			.args(&args)
			.spawn()
			.expect("what");
	}
}
