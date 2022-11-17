pub mod time_fn {
	#![allow(warnings)]

	// PACKAGES
	use std::process::Command;


	fn del_newline(s: &mut String)
	{
		if s.ends_with('\n') || s.ends_with('\r') { s.pop();
			if s.ends_with('\r') || s.ends_with('\n') { s.pop();
				if s.ends_with(' ') { s.pop(); } } }
	} 


	fn get_date() -> String
	{
		let foo = if cfg!(target_os = "windows") {
			Command::new("cmd")
				.args(["/C", "DATE /t"])
				.output().expect("failed to execute process")
		} else {
			Command::new("sh")
				.args(["-c", "date +'%e.%m.%Y'"])
				.output().expect("failed to execute process")
		};

		let mut new_str = String::from_utf8(foo.stdout.clone()).unwrap();

		del_newline(&mut new_str);

		new_str
	}


	fn get_time() -> String
	{
		let foo = if cfg!(target_os = "windows") {
			Command::new("cmd")
				.args(["/C", "TIME /t"])
				.output().expect("failed to execute process")
		} else {
			Command::new("sh")
				.args(["-c", "date +'%H.%M.%S'"])
				.output().expect("failed to execute process")
		};

		let mut new_str = String::from_utf8(foo.stdout.clone()).unwrap();

		del_newline(&mut new_str);

		new_str
	}

	pub fn start() -> [String; 2] {
		let res: [String; 2] = [get_date(), get_time()];
		return res;
	}
}