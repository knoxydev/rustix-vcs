#![allow(warnings)]

use std::env;

extern crate chrono;
use chrono::Utc;

use std::fs;
use std::fs::{File, OpenOptions};

extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};


fn create_yaml() {
	let cwd = env::current_dir().unwrap();
	let folder: String = String::from(cwd.to_string_lossy());

	let os_name = String::from(env::consts::OS);
	let created_date = String::from(Utc::now().format("%Y.%m.%d").to_string());
	let created_time = String::from(Utc::now().format("%H:%M:%S").to_string());

	let info = format!(
"
- info:
	name: {folder}
	os_name: {os_name}
	created_date: {created_date}
	created_time: {created_time}
"
	);

	fs::write("rustix/init.yaml", info).expect("Unable to write file");
}

fn read_yaml() {
	// let docs = YamlLoader::load_from_str(&info).unwrap();
	// let doc = &docs[0]; // select the first document
	// //assert_eq!(doc[0].as_i64().unwrap(), 1); // access elements by index
	
	// let mut out_str = String::new();
	// let mut emitter = YamlEmitter::new(&mut out_str);
	// println!("{:?}", emitter.dump(doc).unwrap());
}

fn init() {
	match fs::create_dir("rustix") {
		Err(why) => println!("! {:?}", why.kind()),
		Ok(_) => {
			match File::create("rustix/init.yaml") {
				Err(error) => println!("! {:?}", error.kind()),
				Ok(_) => create_yaml(),
			}
		},
	}
}

fn main() {
	let action = std::env::args().nth(1).expect("no pattern given");

	match action.as_ref() {
		"init" => init(),
		"exit" => return,
		_ => {
			println!("Incorrect command !");
			return;
		},
	}
}
