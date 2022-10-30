pub mod select_fn {
	#![allow(warnings)]

	// PACKAGES
	use std::fs;
	use std::fs::File;
	use std::io::Write;
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
		save_name: String,
		file_path: String
	}


	fn check_save_exist(save_name: &String) -> Result<[String; 4], Box<dyn Error>>
	{
		let conn = Connection::open("rustix/storage.db3")?;
		let mut stmt = conn.prepare("SELECT id, save_name, file_path FROM main")?;
		let mut base = stmt.query_map(NO_PARAMS, |row| { Ok(DBStr { id: row.get(0)?, save_name: row.get(1)?, file_path: row.get(2)?, }) })?;

		let mut data: [String; 4] = ["".to_string(), "0".to_string(), "".to_string(), "".to_string()];

		for e in base.into_iter() {
			let x = e.unwrap();
			if &x.save_name == save_name {
				data[0] = format!("{:?}", x.id); // ID
				data[1] = "1".to_string(); // EXIST {1 = YES, 0 = NO}
				data[2] = x.file_path; // FILE PATH
				data[3] = x.save_name; // NAME

				break;
			} else { continue; }
		}

		Ok(data)
	}


	fn write_content(file_path: &String, file_name: &String)
	{
		let new_file_name = format!("rustix/saves/{:02}.txt", file_name);
		match fs::read_to_string(new_file_name) {
			Err(error) => println!("{:?}", error.kind()),
			Ok(x) => {
				let mut f = File::create(file_path).expect("Unable to create file");;
    		f.write_all(x.as_bytes()).expect("Unable to write data");

    		println!("To a file {:?} moved saving {:?}", file_path, file_name);
			},
		}
	}

	// START POINT
	pub fn start(save_name: String) {
		crate::log::logger::start("SELECT".to_string());

		let db_str = check_save_exist(&save_name).unwrap();
		if db_str[1] == "0" { return println!("there is no save with this name !"); }

		write_content(&db_str[2], &db_str[3]);
	}
}