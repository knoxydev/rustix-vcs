pub mod add_fn {
	#![allow(warnings)]

	// PACKAGES
	use std::path::Path;
	use std::error::Error;

	use chrono::{Local, DateTime, Utc, Timelike};
	
	extern crate rusqlite;
	use rusqlite::{params, Connection, Result, NO_PARAMS};

	extern crate base64;
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

	//&base64::encode(String::from(txt).into_bytes());


	// FUNC for UPDATE DATABASE - CLEAR DB and THEN INSERT NEW DB
	fn rewrite_db(base: Vec<FileStr>) -> Result<(), Box<dyn Error>> {
		let conn = Connection::open("rustix/storage.db3")?;
		conn.execute("DROP TABLE IF EXISTS main", NO_PARAMS)?;

		conn.execute("CREATE TABLE IF NOT EXISTS main (
			id  				INTEGER 	PRIMARY KEY,
			file_path  	TEXT 			UNIQUE,
			file_name  	TEXT 			NOT NULL,
			saved_date  TEXT      NOT NULL,
			saved_time 	TEXT      NOT NULL,
			main        TEXT      NOT NULL)", NO_PARAMS,
		)?;

		for x in base {
			conn.execute("INSERT INTO main (
				id, file_path, file_name, saved_date, saved_time, main) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
				params![x.id, x.file_path, x.file_name, x.saved_date, x.saved_time, x.main],
			)?;
		}

		conn.close();
		Ok(())
	}


	// FUNC for REWRITE ELEMENTS' ID and ADD NEW ELEMENT
	fn rewrite_id(mut data: FileStr) -> Result<Vec<FileStr>> {
		let conn = Connection::open("rustix/storage.db3")?;
		let mut stmt = conn.prepare("SELECT * FROM main")?;

		let mut base = stmt.query_map(NO_PARAMS, |row| {
			Ok(FileStr {
				id: row.get(0)?,
				file_path: row.get(1)?,
				file_name: row.get(2)?,
				saved_date: row.get(3)?,
				saved_time: row.get(4)?,
				main: row.get(5)?,
			})
		})?;
		
		let mut new_base: Vec<FileStr> = Vec::new();
		let mut new_id: i64 = 1;
		for one in base.into_iter() { new_base.push(one.unwrap()); }

		for e in new_base.iter_mut() {
			e.id = new_id;
			new_id += 1;
		}

		data.id = new_id;
		new_base.push(data);

		Ok(new_base)
	}


	pub fn start(file_path: &String) {
		let name = Path::new(&file_path).file_name().unwrap();
		let now: DateTime<Local> = Local::now();
		let time = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());

		let file_info = FileStr {
			id: 0,
			file_path: file_path.to_string(),
			file_name: name.to_os_string().into_string().unwrap(),
			saved_date: String::from(Utc::now().format("%d.%m.%Y").to_string()),
			saved_time: String::from(&time),
			main: "false".to_string()
		};

		rewrite_db(rewrite_id(file_info).unwrap());

		println!("Added !");
	}
}