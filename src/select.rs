pub mod select_fn
{
	// START POINT
	pub fn start(save_name: String)
	{
		let save_info : (bool, String) = crate::database::slc::start(&save_name);

		if save_info.0 == true {
			crate::database::slc::write_content(&save_info.1, &save_name);
			crate::log::logger::start("SELECT".to_string());
		} else {
			println!("there is no save with this name !");
			crate::log::logger::start("SELECT -> ERROR".to_string());
		}
	}
}