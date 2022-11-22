pub mod get
{
	// PACKAGES
	use std::fs::File;
	use std::io::{prelude::*, BufReader};


	const STORAGE_PATH : &str = "rustix/storage.txt";


	pub fn start() -> std::io::Result<Vec<[String; 4]>>
	{
		fn get_saves(x: String) -> [String; 4]
		{
			let mut elem_start_point : i64 = 0;
			let mut save_info : [String; 4] = ["".to_string(), "".to_string(), "".to_string(), "".to_string()];

			for i in x.chars()
			{
				if i == '|' { elem_start_point += 1; }
				if elem_start_point == 1 { save_info[0].push(i); }
				if elem_start_point == 2 { save_info[1].push(i); }
				if elem_start_point == 3 { save_info[2].push(i); }
				if elem_start_point == 4 { save_info[3].push(i); }
			}
			for i in save_info.iter_mut() { i.remove(0); }

			return save_info;
		}

		let file = File::open(STORAGE_PATH)?;
		let reader = BufReader::new(file);
		let mut saves_base = Vec::new();

		for line in reader.lines() { saves_base.push(get_saves(line?)); }

		Ok(saves_base)
	}
}


pub mod add
{
	// PACKAGES
	use std::path::Path;
	use std::io::prelude::*;
	use std::fs::OpenOptions;


	const STORAGE_PATH : &str = "rustix/storage.txt";


	fn write_file(save_info: [String; 4]) -> bool
	{
		let saves_base = crate::database::get::start().unwrap();
		let mut unique_save : bool = true;

		for i in saves_base { if i[1] == save_info[1] { unique_save = false; } }

		if unique_save == true {
			let res = format!("|{}|{}|{}|{}|", save_info[0], save_info[1], save_info[2], save_info[3]);
			let mut file = OpenOptions::new().write(true).append(true).open(STORAGE_PATH).unwrap();
			writeln!(file, "{}", res);

			println!("Added !");
		} else { println!("Such a name already exists !"); }

		return unique_save;
	}


	// START POINT
	pub fn start(save_info: [String; 4]) -> bool
	{ if Path::new(STORAGE_PATH).exists() == true { return write_file(save_info); } else { return false; } }
}


pub mod del
{
	// PACKAGES
	use std::fs;
	use std::io::prelude::*;
	use std::fs::OpenOptions;


	const STORAGE_PATH : &str = "rustix/storage.txt";


	fn checking(mut base: Vec<[String; 4]>, save_name: String)
	{
		let mut exist_save : bool = false;
		let mut save_id : usize = 0;

		for (i, el) in base.iter().enumerate() {
			if el[1] == save_name { exist_save = true; save_id = i; break; }
		}

		if exist_save == true
		{
			base.remove(save_id);
			
			{ // BLOCK FOR DELETE SAVE FILE
				let save_path = format!("rustix/saves/{:02}.txt", save_name);

				match fs::remove_file(save_path) {
					Err(error) => println!("{:?}", error.kind()),
					Ok(_) => println!("The save was successfully deleted !"),
				}
			}

			// CODE FOR MAKE STORAGE.TXT EMPTY
			OpenOptions::new().write(true).truncate(true).open(STORAGE_PATH);

			{ // BLOCK FOR ADD NEW INFO ABOUT SAVES IN STORAGE.TXT
				let mut file = OpenOptions::new().write(true).append(false).open(STORAGE_PATH).unwrap();

				for i in base {
					let res = format!("|{}|{}|{}|{}|", i[0], i[1], i[2], i[3]);
					writeln!(file, "{}", res);
				}
			}
		} else { println!("some error"); }
	}


	// START POINT
	pub fn start(save_name: String)
	{ checking(crate::database::get::start().unwrap(), save_name); }
}


pub mod slc
{
	pub fn start(save_name: &String) -> (bool, String)
	{
		let saves_base = crate::database::get::start().unwrap();
		let mut save_info : (bool, String) = (false, "".to_string());

		for i in saves_base {
			if i[1] == *save_name {
				save_info.0 = true;
				save_info.1 = i[0].to_string();
			}
		}

		return save_info;
	}
}