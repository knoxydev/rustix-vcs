pub mod init_fn {
	#![allow(warnings)]

	// PACKAGES
	use std::env;
	use std::fs;
	use std::fs::{File, OpenOptions};

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


	fn create_db() -> Result<()>
	{
		let conn = Connection::open("rustix/storage.db3")?;

		conn.execute("CREATE TABLE IF NOT EXISTS main (
			id INTEGER PRIMARY KEY,
			file_path TEXT NOT NULL, save_name TEXT UNIQUE,
			saved_date TEXT NOT NULL, saved_time TEXT NOT NULL)", NO_PARAMS,
		)?;

		conn.close();

		Ok(())
	}


	fn create_yaml()
	{
		let TIME_DATE: [String; 2] = crate::time::time_fn::start();

		let cwd = env::current_dir().unwrap();
		let folder: String = String::from(cwd.to_string_lossy());
		let os_name = String::from(env::consts::OS);
		let created_date = &TIME_DATE[0];
		let created_time = &TIME_DATE[1];

		let info = format!("name: {folder}\nos_name: {os_name}\ncreated_date: {created_date}\ncreated_time: {created_time}");

		fs::write("rustix/init.yml", info).expect("Unable to write file");

		println!("Initialized !");
	}


	// START POINT
	pub fn start() {
		match fs::create_dir("rustix") {
			Err(why) => println!("{:?} !", why.kind()),
			Ok(_) => {
				File::create("rustix/log.txt");
				File::create("rustix/init.yml");
				fs::create_dir("rustix/saves");
				create_yaml();
				create_db();
			},
		}

		crate::log::logger::start("INIT  ".to_string());
	}
}