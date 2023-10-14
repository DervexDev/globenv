#[cfg(target_family = "unix")]
#[cfg(test)]
mod tests {
	#[test]
	fn add_var() {
		globenv::set_var("test", "123").unwrap();
	}

	#[test]
	fn remove_var() {
		globenv::set_var("test", "").unwrap();
	}

	#[test]
	fn read_var() {
		println!(
			"{:?}",
			globenv::get_var("test")
				.unwrap()
				.unwrap_or_else(|| String::from("None"))
		);
	}

	#[test]
	fn set_read_var() {
		globenv::set_var("test", "123").unwrap();
		println!(
			"{:?}",
			globenv::get_var("test")
				.unwrap()
				.unwrap_or_else(|| String::from("None"))
		);
	}

	#[test]
	fn update_var() {
		globenv::set_var("test", "123").unwrap();
		globenv::set_var("test", "456").unwrap();
	}
}
