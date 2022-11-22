pub mod logger
{
	use std::fs::OpenOptions;
	use std::io::prelude::*;

	pub fn start(action: String)
	{
		let time_date: [String; 2] = crate::time::time_fn::start();

		let created_date = &time_date[0];
		let created_time = &time_date[1];

		let info = format!("{} | {} - {}", action, created_date, created_time);

		let mut file = OpenOptions::new().write(true).append(true)
			.open("rustix/log.txt").unwrap();

		if let Err(e) = writeln!(file, "{}", info) { println!("Couldn't write to file: {}", e); }
	}
}