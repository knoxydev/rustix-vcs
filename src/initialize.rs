pub mod init_fn
{
	// PACKAGES
	use std::env;
	use std::fs;


	fn create_yaml()
	{
		let time_date: [String; 2] = crate::time::time_fn::start();

		let cwd = env::current_dir().unwrap();
		let folder: String = String::from(cwd.to_string_lossy());
		let os_name = String::from(env::consts::OS);
		let created_date = &time_date[0];
		let created_time = &time_date[1];

		let info = format!("name: {folder}\nos_name: {os_name}\ncreated_date: {created_date}\ncreated_time: {created_time}");

		fs::write("rustix/init.yml", info).expect("Unable to write file");

		println!("Initialized !");
	}


	// START POINT
	pub fn start() {
		match fs::create_dir("rustix") {
			Err(why) => println!("{:?} !", why.kind()),
			Ok(_) => {
				fs::File::create("rustix/log.txt");
				fs::File::create("rustix/init.yml");
				fs::File::create("rustix/storage.txt");
				fs::create_dir("rustix/saves");
				create_yaml();
			},
		}

		crate::log::logger::start("INIT  ".to_string());
	}
}