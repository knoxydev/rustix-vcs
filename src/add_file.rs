pub mod add_fn {
	#![allow(warnings)]

	// PACKAGES
	use std::path::Path;
	use std::error::Error;

	use chrono::{Local, DateTime, Utc, Timelike};
	
	extern crate rusqlite;
	use rusqlite::{params, Connection, Result};
	// PACKAGES

	#[derive(Debug)]
	struct FileStr {
		id: i64,
		file_path: String,
		file_name: String,
		saved_date: String,
		saved_time: String,
		main: String
	}

	fn add_data(data: FileStr) -> Result<(), Box<dyn Error>> {
		let conn = Connection::open("rustix/storage.db3")?;

		conn.execute("INSERT INTO main (
			id, file_path, file_name, saved_date, saved_time, main) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
			params![&data.id, &data.file_path, &data.file_name, &data.saved_date, &data.saved_time, &data.main],
		)?;

		conn.close();

		Ok(())
	}

	pub fn start(file_path: &String) {
		let name = Path::new(&file_path).file_name().unwrap();
		let now: DateTime<Local> = Local::now();
		let time = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());

		let file_info = FileStr {
			id: 2,
			file_path: file_path.to_string(),
			file_name: name.to_os_string().into_string().unwrap(),
			saved_date: String::from(Utc::now().format("%d.%m.%Y").to_string()),
			saved_time: String::from(&time),
			main: "false".to_string()
		};

		add_data(file_info);

		println!("Added !");
	}
}