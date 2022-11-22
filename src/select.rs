pub mod select_fn
{
	// PACKAGES
	use std::fs;
	use std::io::Write;


	fn write_content(file_path: &String, file_name: &String)
	{
		let new_file_name = format!("rustix/saves/{:02}.txt", file_name);
		match fs::read_to_string(new_file_name) {
			Err(error) => println!("{:?}", error.kind()),
			Ok(x) => {
				let mut f = fs::File::create(file_path).expect("Unable to create file");
    		f.write_all(x.as_bytes()).expect("Unable to write data");

    		println!("To a file {:?} moved saving {:?}", file_path, file_name);
			},
		}
	}


	// START POINT
	pub fn start(save_name: String)
	{
		crate::log::logger::start("SELECT".to_string());

		let save_info : (bool, String) = crate::database::slc::start(&save_name);

		if save_info.0 == true { write_content(&save_info.1, &save_name); }
		else { println!("there is no save with this name !"); }
	}
}