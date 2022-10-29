pub mod print_fn {
	#![allow(warnings)]

	// PACKAGES
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
	pub fn start() { print_db(); }
}