pub mod print_fn
{
	// PACKAGES
	use std::fs::File;
	use std::error::Error;
	use std::io::{prelude::*, BufReader};

	use serde::{Deserialize, Serialize};
	use serde_yaml::{self};
	// PACKAGES


	#[derive(Debug, Serialize, Deserialize)]
	struct Config {
		name: String,
		os_name: String,
		created_date: String,
		created_time: String,
	}


	pub fn print_commands() {
		println!("Commands:\n");

		println!("rustix-vcs init\n  command to initialize the project\n");
		println!("rustix-vcs add src/main.js save_name\n  command to save the file.\n");
		println!("rustix-vcs delete save_name\n  command to delete the save.\n");
		println!("rustix-vcs select save_name\n  command to insert saved content into a file.\n");
		println!("rustix-vcs print\n  command to display all saves and info about project.\n");
		println!("rustix-vcs info\n  command to view information about the initialized project.\n");
		println!("rustix-vcs cmd\n  command to display all commands.\n");
		println!("rustix-vcs log\n  command to view logs.\n");
	}


	fn show_log() -> std::io::Result<()> {
		let file = File::open("rustix/log.txt")?;
		let reader = BufReader::new(file);
		for line in reader.lines() { println!("{}", line?); }

		Ok(())
	}


	pub fn read_yaml() {
		let f = File::open("rustix/init.yml").expect("Could not open file.");
		let mut data: Config = serde_yaml::from_reader(f).expect("Couldn't read");
		
		println!("INFO\n  os: {}\n  created date: {} - {}\n  current path: {}\n\n",
			data.os_name, data.created_date, data.created_time, data.name);
	}


	fn print_db() {
		let SAVES_BASE = crate::database::get::start().unwrap();
		let mut id : i64 = 1;

		for x in SAVES_BASE.into_iter() {
			println!("{}. {}\n   - path\n      {}\n   - saved\n      {}\n      {}\n",
			id, x[1], x[0], x[2], x[3]);

			id += 1;
		}
	}


	// START POINT
	pub fn start(x: i64) {
		if x == 1 { show_log(); }
		else {
			crate::log::logger::start("PRINT ".to_string());
			read_yaml();
			print_db();
		}
	}
}