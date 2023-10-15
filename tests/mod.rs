#[cfg(test)]
mod tests {
	#[test]
	fn set_var() {
		globenv::set_var("test", "123").unwrap();
	}

	#[test]
	fn remove_var() {
		globenv::set_var("test", "").unwrap();
	}

	#[test]
	fn get_var() {
		println!(
			"{:?}",
			globenv::get_var("PATH")
				.unwrap()
				.unwrap_or_else(|| String::from("None"))
		);
	}

	#[test]
	fn set_get_var() {
		globenv::set_var("test", "123").unwrap();
		println!(
			"{:?}",
			globenv::get_var("test")
				.unwrap()
				.unwrap_or_else(|| String::from("None"))
		);
	}

	#[test]
	fn set_set_var() {
		globenv::set_var("test", "123").unwrap();
		globenv::set_var("test", "456").unwrap();
	}

	#[test]
	fn set_path() {
		globenv::set_path("test/path").unwrap();
	}

	#[test]
	fn get_paths() {
		println!(
			"{:?}",
			globenv::get_paths()
				.unwrap()
				.unwrap_or_else(|| String::from("None"))
		);
	}

	#[test]
	fn remove_path() {
		globenv::remove_path("test/path").unwrap();
	}

	#[test]
	fn add_get_path() {
		globenv::set_path("test/path").unwrap();
		println!("{:?}", globenv::get_paths().unwrap().unwrap());
	}

	#[test]
	fn remove_get_path() {
		globenv::remove_path("test/path").unwrap();
		println!("{:?}", globenv::get_paths().unwrap().unwrap());
	}
}
