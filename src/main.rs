#![allow(warnings)]

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
		"commands" => {
			println!("\nrustix-vcs init -> command to initialize the project");
			println!("\nrustix-vcs add 'path_to_file' -> command to save the file");
			println!("\nrustix-vcs select 'save_name' -> command to insert saved content into a file\n");
		},
		"print" => print_fn::start(0),
		"log" => print_fn::start(1),
		"info" => print_fn::read_yaml(),
		"exit" => return,
		_ => {
			println!("Incorrect command !");
			return;
		},
	}
}
