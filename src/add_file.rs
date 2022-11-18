pub mod add_fn {
	#![allow(warnings)]

	// PACKAGES
	use std::fs;
	use std::io::Write;
	use std::path::Path;
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

	#[derive(Debug)]
	struct DBStr {
		id: i64,
		save_name: String
	}


	// FUNC for UPDATE DATABASE - CLEAR DB and THEN INSERT NEW DB
	fn rewrite_db(base: Vec<FileStr>) -> Result<(), Box<dyn Error>> {
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


	// FUNC for REWRITE ELEMENTS' ID and ADD NEW ELEMENT
	fn rewrite_id(mut data: FileStr) -> Result<Vec<FileStr>> {
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
		for one in base.into_iter() { new_base.push(one.unwrap()); }

		for e in new_base.iter_mut() {
			e.id = new_id;
			new_id += 1;
		}

		data.id = new_id;
		new_base.push(data);

		Ok(new_base)
	}


	// COPY FILE'S CONTENT and CREATE SAVE
	fn create_save(unq_name: &String, file_path: &String) {
		fn copy_file(file_path: &String) -> String { return fs::read_to_string(file_path).expect("Should have been able to read the file"); }

		let file_name = format!("rustix/saves/{:02}.txt", unq_name);
		let data = copy_file(&file_path);
		let new_data = data.as_bytes();

    let mut f = fs::File::create(file_name).expect("Unable to create file");
    f.write_all(new_data).expect("Unable to write data");
	}


	// START POINT
	pub fn start(file_path: &String, unq_name: &String) {
		crate::log::logger::start("ADD   ".to_string());
		let TIME_DATE: [String; 2] = crate::time::time_fn::start();

		fn check_unique_name(unq: &String) -> Result<bool, Box<dyn Error>>
		{
			let conn = Connection::open("rustix/storage.db3")?;
			let mut stmt = conn.prepare("SELECT id, save_name FROM main")?;
			let mut base = stmt.query_map(NO_PARAMS, |row| { Ok(DBStr { id: row.get(0)?, save_name: row.get(1)?, }) })?;

			let mut x: bool = false;
			for e in base.into_iter() {
				if &e.unwrap().save_name == unq {
					println!("Such a name already exists !");
					x = true;

					break;
				} else { continue; }
			}

			Ok(x)
		}
		if check_unique_name(&unq_name).unwrap() == true { return; }


		let file_info = FileStr {
			id: 0,
			file_path: file_path.to_string(),
			save_name: unq_name.to_string(),
			saved_date: TIME_DATE[0].to_string(),
			saved_time: TIME_DATE[1].to_string()
		};

		create_save(&unq_name, &file_path);
		rewrite_db(rewrite_id(file_info).unwrap());

		println!("Added !");
	}
}