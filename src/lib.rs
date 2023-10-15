//             __      __
//      ____ _/ /___  / /_  ___  ____ _   __
//     / __  / / __ \/ __ \/ _ \/ __ \ | / /
//    / /_/ / / /_/ / /_/ /  __/ / / / |/ /
//    \___ /_/\____/_____/\___/_/ /_/|___/
//   /____/
//
//! # globenv
//!
//! Globally set & read environment variables and paths on Windows, macOS or Linux.
//!
//! ## Example:
//! ```rust
//! use globenv::{set_var, get_var};
//! // Environment Variables
//! get_var("key").unwrap();
//! set_var("key", "value").unwrap();
//! remove_var("key").unwrap();
//! // Environment Paths
//! get_paths().unwrap();
//! set_path("example/path").unwrap();
//! remove_var("example/path").unwrap();
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
fn get_env() -> Result<(String, PathBuf), EnvError> {
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

	Ok((env, env_dir))
}

#[cfg(target_family = "unix")]
/// Gets the environment variable from the current process or global environment.
pub fn get_var(key: &str) -> Result<Option<String>, EnvError> {
	let var = env::var(&key);

	if var.is_ok() {
		return Ok(Some(var.unwrap()));
	}

	let (env, _) = get_env()?;

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

#[cfg(target_family = "unix")]
/// Sets a environment variable globally and in the current process.
pub fn set_var(key: &str, value: &str) -> Result<(), EnvError> {
	let (env, env_dir) = get_env()?;

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

	Ok(())
}

#[cfg(target_family = "unix")]
/// Removes the environment variable globally and from the current process.
pub fn remove_var(key: &str) -> Result<(), EnvError> {
	let (env, env_dir) = get_env()?;

	let mut export = String::from("export ");
	export.push_str(key);

	if !env.contains(&export) {
		env::remove_var(key);
		return Ok(());
	}

	let mut updated_env = String::new();
	for line in env.lines() {
		if !line.contains(&export) {
			updated_env.push_str(line);
			updated_env.push_str("\n");
		}
	}

	fs::write(env_dir, updated_env)?;
	env::remove_var(key);

	Ok(())
}

#[cfg(target_family = "unix")]
/// Gets all environment paths from the current process.
pub fn get_paths() -> Result<Option<String>, EnvError> {
	let path = env::var("PATH");

	if path.is_ok() {
		return Ok(Some(path.unwrap()));
	}

	Ok(None)
}

#[cfg(target_family = "unix")]
/// Adds a environment path globally and in the current process.
pub fn set_path(path: &str) -> Result<(), EnvError> {
	let (mut env, env_dir) = get_env()?;

	let mut export = String::from("export PATH=");
	export.push_str(&path);
	export.push_str("\n");

	env.push_str(&export);

	let mut var = env::var("PATH")?;
	var.push_str(":");
	var.push_str(&path);

	fs::write(env_dir, env)?;
	env::set_var("PATH", var);

	Ok(())
}

#[cfg(target_family = "unix")]
/// Removes the environment path globally and from the current process.
pub fn remove_path(path: &str) -> Result<(), EnvError> {
	let (env, env_dir) = get_env()?;

	let mut updated_env = String::new();
	let mut export = String::from("export PATH=");
	export.push_str(&path);

	for line in env.lines() {
		if !line.contains(&export) {
			updated_env.push_str(line);
			updated_env.push_str("\n");
		}
	}

	let mut prefix = String::from(":");
	prefix.push_str(path);

	let mut suffix = String::from(path);
	suffix.push_str(":");

	let mut var = env::var("PATH")?;
	var = var.replace(&prefix, "");
	var = var.replace(&suffix, "");

	fs::write(env_dir, updated_env)?;
	env::set_var("PATH", var);

	Ok(())
}

#[cfg(target_os = "windows")]
/// Sets a environment variable globally and in the current process. Empty value removes variable completely.
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
/// Gets the environment variable from the current process or global environment.
pub fn get_var(key: &str) -> Result<Option<String>, EnvError> {
	let var = env::var(&key);

	if var.is_ok() {
		return Ok(Some(var.unwrap()));
	}

	let current_user = RegKey::predef(HKEY_CURRENT_USER);
	let subkey = current_user.open_subkey_with_flags("Environment", KEY_READ)?;
	let value: Result<String, std::io::Error> = subkey.get_value(&key);

	if value.is_ok() {
		return Ok(Some(value.unwrap()));
	}

	Ok(None)
}
