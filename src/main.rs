#![allow(warnings)]

// PACKAGES
use std::io;

// MODULES
mod initialize;
pub use crate::initialize::init_fn;

mod add_file;
pub use crate::add_file::add_fn;

mod delete;
pub use crate::delete::delete_fn;

mod select;
pub use crate::select::select_fn;

mod print;
pub use crate::print::print_fn;

mod log;
// MODULES


fn main() {
	{
		let args: Vec<_> = std::env::args().collect();

		if args.len() == 1 {
			println!("You should enter arguments to use the program.\n");

			print_fn::print_commands();

			io::stdin().read_line(&mut String::new()).unwrap();
			return;
		}
	}

	let first_arg = std::env::args().nth(1).expect("no pattern given");

	match first_arg.as_ref() {
		"init" => init_fn::start(),
		"add" => {
			let args: Vec<_> = std::env::args().collect();

			if args.len() > 3 {
				let scd_arg = std::env::args().nth(2).expect("no pattern given");
				let thd_arg = std::env::args().nth(3).expect("no pattern given");

				let fl_exist = std::path::Path::new(&scd_arg).exists();

				if fl_exist == true { add_fn::start(&scd_arg, &thd_arg); }
				else { println!("not exist"); }
			}
			else
			{ println!("where is third argument ?"); }
		},
		"delete" => {
			let scd_arg = std::env::args().nth(2).expect("no pattern given");
			delete_fn::start(scd_arg);
		},
		"select" => {
			let scd_arg = std::env::args().nth(2).expect("no pattern given");
			select_fn::start(scd_arg);
		},
		"print" => print_fn::start(0),
		"log" => print_fn::start(1),
		"info" => print_fn::read_yaml(),
		"Commands" => print_fn::print_commands(),
		"exit" => return,
		_ => {
			println!("Incorrect command !");
			return;
		},
	}
}
