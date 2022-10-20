pub mod init_fn {
	#![allow(warnings)]

	// PACKAGES
	use std::env;
	use std::fs;
	use std::fs::{File, OpenOptions};

	use chrono::{Local, DateTime, Utc, Timelike};

	use serde::{Deserialize, Serialize};
	use serde_yaml::{self};

	extern crate rusqlite;
	use rusqlite::{params, Connection, Result, NO_PARAMS};
	// PACKAGES

	#[derive(Debug, Serialize, Deserialize)]
	struct Config {
		name: String,
		os_name: String,
		created_date: String,
		created_time: String,
	}

	fn create_db() -> Result<()> {
		let conn = Connection::open("rustix/storage.db3")?;

		conn.execute("CREATE TABLE IF NOT EXISTS main (
			id  				INTEGER 	PRIMARY KEY,
			file_path  	TEXT 			UNIQUE,
			file_name  	TEXT 			NOT NULL,
			saved_date  TEXT      NOT NULL,
			saved_time 	TEXT      NOT NULL,
			main        TEXT      NOT NULL)", NO_PARAMS,
		)?;

		conn.close();

		Ok(())
	}

	fn create_yaml() {
		let cwd = env::current_dir().unwrap();
		let now: DateTime<Local> = Local::now();
		let time = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());

		let folder: String = String::from(cwd.to_string_lossy());
		let os_name = String::from(env::consts::OS);
		let created_date = String::from(Utc::now().format("%d.%m.%Y").to_string());
		let created_time = String::from(&time);

		let info = format!("name: {folder}\nos_name: {os_name}\ncreated_date: {created_date}\ncreated_time: {created_time}");

		fs::write("rustix/init.yml", info).expect("Unable to write file");

		println!("Initialized !");
	}

	fn read_yaml() {
		let f = File::open("rustix/init.yml").expect("Could not open file.");
		let mut scrape_config: Config = serde_yaml::from_reader(f).expect("Couldn't read");
		println!("{:?}", scrape_config);
	}

	pub fn start() {
		match fs::create_dir("rustix") {
			Err(why) => {
				println!("{:?} !", why.kind());
				read_yaml();
			},
			Ok(_) => {
				match File::create("rustix/init.yml") {
					Err(error) => println!("{:?}", error.kind()),
					Ok(_) => {
						create_yaml();
						create_db();

						fs::create_dir("rustix/saves");
					},
				}
			},
		}
	}
}