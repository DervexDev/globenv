//             __      __
//      ____ _/ /___  / /_  ___  ____ _   __
//     / __  / / __ \/ __ \/ _ \/ __ \ | / /
//    / /_/ / / /_/ / /_/ /  __/ / / / |/ /
//    \___ /_/\____/_____/\___/_/ /_/|___/
//   /____/
//
//! # globenv
//!
//! Globally set and read environment variables on Windows, macOS and Linux.
//!
//! ## Example:
//! ```rust
//! use globenv::{set_var, get_var};
//! // Set variable
//! set_var("key", "value").unwrap();
//! // Remove variable
//! set_var("key", "").unwrap();
//! // Read variable
//! get_var("key").unwrap();
//! ```

use std::{env, error, fmt};

#[cfg(target_family = "unix")]
use std::{fs, path::PathBuf};

#[cfg(target_os = "windows")]
use winreg::{enums::*, RegKey};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnvError {
	/// Unsupported shell
	ShellError,
	/// IO Error (file or registry operation)
	IOError,
	/// Var error (can't get or set variable)
	VarError,
}

impl error::Error for EnvError {}

impl From<std::io::Error> for EnvError {
	fn from(_: std::io::Error) -> EnvError {
		EnvError::IOError
	}
}

impl From<std::env::VarError> for EnvError {
	fn from(_: std::env::VarError) -> EnvError {
		EnvError::VarError
	}
}

impl fmt::Display for EnvError {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_str(match self {
			EnvError::ShellError => "Error: unsupported shell",
			EnvError::IOError => "Error: failed to perform I/O operation",
			EnvError::VarError => "Error: failed to get or set env variable",
		})
	}
}

#[cfg(target_family = "unix")]
/// Sets a global environment variable globally and in the current process. Empty value removes environment variable completely.
pub fn set_var(key: &str, value: &str) -> Result<(), EnvError> {
	let home_dir = env::var("HOME")?;
	let shell_dir = match env::var("SHELL")?.as_str() {
		"/usr/bin/zsh" => ".zshenv",
		"/bin/zsh" => ".zshenv",
		"/bin/bash" => ".bashrc",
		_ => return Err(EnvError::ShellError),
	};

	let mut env_dir = PathBuf::from(home_dir);
	env_dir.push(shell_dir);

	let env = fs::read_to_string(&env_dir)?;

	// Set a new env variable
	if !value.is_empty() {
		let mut updated_env = String::new();

		let mut export = String::from("export ");
		export.push_str(key);
		export.push_str("=");

		for line in env.lines() {
			if !line.contains(&export) {
				updated_env.push_str(line);
				updated_env.push_str("\n");
			}
		}

		export.push_str(value);
		export.push_str("\n");
		updated_env.push_str(&export);

		fs::write(env_dir, updated_env)?;
		env::set_var(key, value);

	// Remove the env variable
	} else {
		let mut export = String::from("export ");
		export.push_str(key);
		export.push_str("=");

		if !env.contains(&export) {
			env::remove_var(key);
			return Ok(());
		}

		let mut updated_env = String::new();

		for line in env.lines() {
			if !line.contains(key) {
				updated_env.push_str(line);
				updated_env.push_str("\n");
			}
		}

		fs::write(env_dir, updated_env)?;
		env::remove_var(key);
	}

	Ok(())
}

#[cfg(target_family = "unix")]
/// Reads the global environment variable.
pub fn get_var(key: &str) -> Result<Option<String>, EnvError> {
	let var = env::var(&key);

	if var.is_ok() {
		return Ok(Some(var.unwrap()));
	}

	let home_dir = env::var("HOME")?;
	let shell_dir = match env::var("SHELL")?.as_str() {
		"/usr/bin/zsh" => ".zshenv",
		"/bin/zsh" => ".zshenv",
		"/bin/bash" => ".bashrc",
		_ => return Err(EnvError::ShellError),
	};

	let mut env_dir = PathBuf::from(home_dir);
	env_dir.push(shell_dir);

	let env = fs::read_to_string(&env_dir)?;

	let mut export = String::from("export ");
	export.push_str(key);
	export.push_str("=");

	if !env.contains(&export) {
		return Ok(None);
	}

	let start = &env[env.find(&export).unwrap() + export.len()..];
	let end = &start[..start.find("\n").unwrap_or_else(|| start.len())];

	Ok(Some(end.to_owned()))
}

#[cfg(target_os = "windows")]
/// Sets a global environment variable globally and in the current process. Empty value removes environment variable completely.
pub fn set_var(key: &str, value: &str) -> Result<(), EnvError> {
	let current_user = RegKey::predef(HKEY_CURRENT_USER);
	let subkey = current_user.open_subkey_with_flags("Environment", KEY_SET_VALUE)?;

	// Set a new env variable
	if !value.is_empty() {
		subkey.set_value(key, &value)?;
		env::set_var(key, value);

	// Remove the env variable
	} else {
		subkey.delete_value(key)?;
		env::remove_var(key);
	}

	Ok(())
}
#[cfg(target_os = "windows")]
/// Reads the global environment variable.
pub fn get_var(key: &str) -> Result<Option<String>, EnvError> {
	let var = env::var(&key);

	if var.is_ok() {
		return Ok(Some(var.unwrap()));
	}

	let current_user = RegKey::predef(HKEY_CURRENT_USER);
	let subkey = current_user.open_subkey_with_flags("Environment", KEY_SET_VALUE)?;

	println!("{:?}", subkey.get_value(&key));

	Ok(())
}
