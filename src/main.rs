#![allow(warnings)]

// PACKAGES
use std::env;

// MODULES
mod initialize;
pub use crate::initialize::init_fn;


fn main() {
	let action = env::args().nth(1).expect("no pattern given");

	match action.as_ref() {
		"init" => init_fn::start(),
		"exit" => return,
		_ => {
			println!("Incorrect command !");
			return;
		},
	}
}
