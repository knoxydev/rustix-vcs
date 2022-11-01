pub mod print_fn {
	#![allow(warnings)]

	// PACKAGES
	use std::fs::File;
	use std::error::Error;
	use std::io::{prelude::*, BufReader};

	extern crate rusqlite;
	use rusqlite::{params, Connection, Result, NO_PARAMS};

	use serde::{Deserialize, Serialize};
	use serde_yaml::{self};
	// PACKAGES

	#[derive(Debug)]
	struct FileStr {
		id: i64,
		file_path: String,
		save_name: String,
		saved_date: String,
		saved_time: String
	}

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
		println!("rustix-vcs log\n  command to view logs.\n");
	}


	fn show_log() -> Result<(), std::io::Error> {
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


	fn print_db() -> Result<(), Box<dyn Error>> {
		let conn = Connection::open("rustix/storage.db3")?;
		let mut stmt = conn.prepare("SELECT * FROM main")?;

		let mut base = stmt.query_map(NO_PARAMS, |row| {
			Ok(FileStr {
				id: row.get(0)?,
				file_path: row.get(1)?, save_name: row.get(2)?,
				saved_date: row.get(3)?, saved_time: row.get(4)?,
			})
		})?;
		
		let mut new_base: Vec<FileStr> = Vec::new();
		for one in base.into_iter() { new_base.push(one.unwrap()); }

		for x in new_base.into_iter() {
			println!("{}. {}\n   - path\n      {}\n   - saved\n      {}\n      {}\n",
			x.id, x.save_name, x.file_path, x.saved_date, x.saved_time);
		}

		Ok(())
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