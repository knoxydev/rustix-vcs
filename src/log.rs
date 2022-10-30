pub mod logger {
	use std::fs::OpenOptions;
	use std::io::prelude::*;

	use chrono::{Local, DateTime, Utc, Timelike};

	pub fn start(action: String) {
		let now: DateTime<Local> = Local::now();
		let time = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
		let created_date = String::from(Utc::now().format("%d.%m.%Y").to_string());
		let created_time = String::from(&time);

		let info = format!("{} | {} - {}", action, created_date, created_time);

		let mut file = OpenOptions::new().write(true).append(true)
			.open("rustix/log.txt").unwrap();

		if let Err(e) = writeln!(file, "{}", info) {
			println!("Couldn't write to file: {}", e);
		}
	}
}