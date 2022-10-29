pub mod delete_fn {
	#![allow(warnings)]

	// PACKAGES
	use std::fs;
	use std::error::Error;
	
	extern crate rusqlite;
	use rusqlite::{params, Connection, Result, NO_PARAMS};
	// PACKAGES

	#[derive(Debug)]
	struct FileStr {
		id: i64,
		file_path: String,
		save_name: String,
		saved_date: String,
		saved_time: String
	}


	// CREATE NEW CLEARED ARRAY with DELETED ELEMENT, and DELETE SAVE FILE
	fn delete_save(save_name: String) -> Result<Vec<FileStr>, Box<dyn Error>> {

		{ // BLOCK FOR DELETE SAVE FILE
			let save_path = format!("rustix/saves/{:02}.txt", save_name);

			match fs::remove_file(save_path) {
				Err(error) => println!("{:?}", error.kind()),
				Ok(_) => println!("The save was successfully deleted !"),
			}
		}

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
		let mut new_id: i64 = 1;

		for x in base.into_iter() {
			if x.as_ref().unwrap().save_name == save_name { continue; }
			else { new_base.push(x.unwrap()); }
		}
		for x in new_base.iter_mut() {
			x.id = new_id;
			new_id += 1;
		}

		Ok(new_base)
	}


	// UPDATE DB and INSERT UPDATED DB
	fn update_db(base: Vec<FileStr>) -> Result<(), Box<dyn Error>> {
		let conn = Connection::open("rustix/storage.db3")?;
		conn.execute("DROP TABLE IF EXISTS main", NO_PARAMS)?;

		conn.execute("CREATE TABLE IF NOT EXISTS main (
			id INTEGER PRIMARY KEY,
			file_path TEXT NOT NULL, save_name TEXT UNIQUE,
			saved_date TEXT NOT NULL, saved_time TEXT NOT NULL)", NO_PARAMS,
		)?;

		for x in base {
			conn.execute("INSERT INTO main (
				id, file_path, save_name, saved_date, saved_time) VALUES (?1, ?2, ?3, ?4, ?5)",
				params![x.id, x.file_path, x.save_name, x.saved_date, x.saved_time],
			)?;
		}

		conn.close();
		Ok(())
	}

	// START POINT
	pub fn start(save_name: String) {
		update_db(delete_save(save_name).unwrap());
	}
}