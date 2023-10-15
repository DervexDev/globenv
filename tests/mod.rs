#[cfg(test)]
mod tests {
	// Environment Variables
	#[test]
	fn get_var() {
		println!(
			"{:?}",
			globenv::get_var("test")
				.unwrap()
				.unwrap_or_else(|| String::from("None"))
		);
	}

	#[test]
	fn set_var() {
		globenv::set_var("test", "123").unwrap();
	}

	#[test]
	fn remove_var() {
		globenv::remove_var("test").unwrap();
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

	// Environment Paths
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
	#[cfg(target_family = "unix")]
	fn set_path() {
		globenv::set_path("$HOME/Desktop").unwrap();
	}

	#[test]
	#[cfg(target_family = "unix")]
	fn remove_path() {
		globenv::remove_path("$HOME/Desktopp").unwrap();
	}

	#[test]
	#[cfg(target_family = "unix")]
	fn set_get_path() {
		globenv::set_path("$HOME/Desktop").unwrap();
		println!(
			"{:?}",
			globenv::get_paths()
				.unwrap()
				.unwrap_or_else(|| String::from("None"))
		);
	}

	#[test]
	#[cfg(target_family = "unix")]
	fn remove_get_path() {
		globenv::remove_path("$HOME/Desktop").unwrap();
		println!(
			"{:?}",
			globenv::get_paths()
				.unwrap()
				.unwrap_or_else(|| String::from("None"))
		);
	}

	#[test]
	#[cfg(target_os = "windows")]
	fn set_path() {
		globenv::set_path("%USERPROFILE%\\Desktop\\temp").unwrap();
	}

	#[test]
	#[cfg(target_os = "windows")]
	fn remove_path() {
		globenv::remove_path("%USERPROFILE%\\Desktop\\temp").unwrap();
	}

	#[test]
	#[cfg(target_os = "windows")]
	fn set_get_path() {
		globenv::set_path("%USERPROFILE%\\Desktop\\temp").unwrap();
		println!(
			"{:?}",
			globenv::get_paths()
				.unwrap()
				.unwrap_or_else(|| String::from("None"))
		);
	}

	#[test]
	#[cfg(target_os = "windows")]
	fn remove_get_path() {
		globenv::remove_path("%USERPROFILE%\\Desktop\\temp").unwrap();
		println!(
			"{:?}",
			globenv::get_paths()
				.unwrap()
				.unwrap_or_else(|| String::from("None"))
		);
	}
}
