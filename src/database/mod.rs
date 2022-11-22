pub mod get
{
	// PACKAGES
	use std::fs::File;
	use std::io::{self, prelude::*, BufReader};


	const storagePATH : &str = "rustix/storage.txt";


	pub fn start() -> std::io::Result<Vec<[String; 4]>>
	{
		fn get_saves(x: String) -> [String; 4]
		{
			let mut elemStartPoint : i64 = 0;
			let mut SAVE_INFO : [String; 4] = ["".to_string(), "".to_string(), "".to_string(), "".to_string()];

			for i in x.chars()
			{
				if (i == '|') { elemStartPoint += 1; }
				if (elemStartPoint == 1) { SAVE_INFO[0].push(i); }
				if (elemStartPoint == 2) { SAVE_INFO[1].push(i); }
				if (elemStartPoint == 3) { SAVE_INFO[2].push(i); }
				if (elemStartPoint == 4) { SAVE_INFO[3].push(i); }
			}
			for i in SAVE_INFO.iter_mut() { i.remove(0); }

			return SAVE_INFO;
		}

		let file = File::open(storagePATH)?;
		let reader = BufReader::new(file);
		let mut SAVES_BASE = Vec::new();

		for line in reader.lines() { SAVES_BASE.push(get_saves(line?)); }

		Ok(SAVES_BASE)
	}
}

pub mod add
{
	// PACKAGES
	use std::path::Path;
	use std::fs::File;
	use std::io::{self, prelude::*, BufReader};

	use std::io::prelude::*;
	use std::fs::OpenOptions;


	const storagePATH : &str = "rustix/storage.txt";


	fn write_file(save_info: [String; 4]) -> bool
	{
		let SAVES_BASE = crate::database::get::start().unwrap();
		let mut UNIQUE_SAVE : bool = true;

		for i in SAVES_BASE { if (i[1] == save_info[1]) { UNIQUE_SAVE = false; } }

		if (UNIQUE_SAVE == true) {
			let res = format!("|{}|{}|{}|{}|", save_info[0], save_info[1], save_info[2], save_info[3]);
			let mut file = OpenOptions::new().write(true).append(true).open(storagePATH).unwrap();
			writeln!(file, "{}", res);

			println!("Added !");
		} else { println!("Such a name already exists !"); }

		return UNIQUE_SAVE;
	}


	// START POINT
	pub fn start(save_info: [String; 4]) -> bool
	{ if (Path::new(storagePATH).exists() == true) { return write_file(save_info); } else { return false; } }
}