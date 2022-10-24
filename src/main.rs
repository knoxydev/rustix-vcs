#![allow(warnings)]

// PACKAGES

// MODULES
mod initialize;
pub use crate::initialize::init_fn;

mod add_file;
pub use crate::add_file::add_fn;


fn main() {
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
		"exit" => return,
		_ => {
			println!("Incorrect command !");
			return;
		},
	}
}
