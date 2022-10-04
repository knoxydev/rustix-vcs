#![allow(warnings)]

use std::fs::File;

fn init() {
	let action = std::env::args().nth(1).expect("no pattern given");

	match resp.as_ref() {
		"init" => {
			let mut file = File::create("test-rustix.txt").expect("Error !");
		},
		"exit" => return,
		_ => {
			println!("Incorrect command !");
			return;
		},
	}
}

fn main() {
	init();
}
