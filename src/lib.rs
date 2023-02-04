// Home file sharing and storage server
// Backend library
// By Colby Reinhart
// 2-4-2023

use std::path::{Path, PathBuf};
use std::fs::read_to_string;

use serde::Deserialize;

// Config.toml model
#[derive(Deserialize)]
pub struct Config
{
	pub server: ServerConfig
}

#[derive(Deserialize)]
pub struct ServerConfig
{
	pub storage_root_loc: PathBuf
}

impl Config
{
	pub fn from_file(filepath: &Path) -> Self
	{
		let file_contents = read_to_string(filepath)
			.expect("Could not open config toml");
		toml::from_str(&file_contents)
			.expect("Could not parse config toml")
	}
}