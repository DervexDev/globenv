#[cfg(test)]
mod tests {
	fn test_var() -> &'static str {
		"test"
	}

	fn test_path() -> &'static str {
		#[cfg(target_family = "unix")]
		return "$HOME/Desktop/test";

		#[cfg(target_os = "windows")]
		return "%USERPROFILE%\\Desktop\\test";
	}

	// Environment Variables
	#[test]
	fn get_var() {
		println!(
			"{:?}",
			globenv::get_var(test_var())
				.unwrap()
				.unwrap_or_else(|| String::from("None"))
		);
	}

	#[test]
	fn set_var() {
		globenv::set_var(test_var(), "123").unwrap();
	}

	#[test]
	fn remove_var() {
		globenv::remove_var(test_var()).unwrap();
	}

	#[test]
	fn set_get_var() {
		globenv::set_var(test_var(), "123").unwrap();
		println!(
			"{:?}",
			globenv::get_var(test_var())
				.unwrap()
				.unwrap_or_else(|| String::from("None"))
		);
	}

	#[test]
	fn remove_get_var() {
		globenv::remove_var(test_var()).unwrap();
		println!(
			"{:?}",
			globenv::get_var(test_var())
				.unwrap()
				.unwrap_or_else(|| String::from("None"))
		);
	}

	#[test]
	fn set_set_var() {
		globenv::set_var(test_var(), "123").unwrap();
		globenv::set_var(test_var(), "456").unwrap();
	}

	// Environment Paths
	#[test]
	fn get_paths() {
		println!(
			"{:?}",
			globenv::get_paths().unwrap_or_else(|| String::from("None"))
		);
	}

	#[test]
	fn set_path() {
		globenv::set_path(test_path()).unwrap();
	}

	#[test]
	fn remove_path() {
		globenv::remove_path(test_path()).unwrap();
	}

	#[test]
	fn set_get_path() {
		globenv::set_path(test_path()).unwrap();
		println!(
			"{:?}",
			globenv::get_paths().unwrap_or_else(|| String::from("None"))
		);
	}

	#[test]
	fn remove_get_path() {
		globenv::remove_path(test_path()).unwrap();
		println!(
			"{:?}",
			globenv::get_paths().unwrap_or_else(|| String::from("None"))
		);
	}

	#[test]
	fn set_remove_get_path() {
		globenv::set_path(test_path()).unwrap();
		globenv::remove_path(test_path()).unwrap();
		println!(
			"{:?}",
			globenv::get_paths().unwrap_or_else(|| String::from("None"))
		);
	}
}
