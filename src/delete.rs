pub mod delete_fn
{
	// START POINT
	pub fn start(save_name: String) {
		crate::log::logger::start("DELETE".to_string());
		crate::database::del::start(save_name);
	}
}