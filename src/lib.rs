// Home file sharing and storage server
// Backend library
// By Colby Reinhart
// 2-4-2023

use std::path::{Path, PathBuf};
use std::fs::{read_to_string, DirEntry};
use serde::{Deserialize, Serialize};

//
// Config.toml model
//

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

//
// Directory entry model
//

#[derive(Serialize)]
pub struct Resource
{
	pub name: String,
	pub path: PathBuf,
	pub is_directory: bool
}

impl Resource
{
	pub fn from(entry: DirEntry, server_route: PathBuf) -> Self
	{
		let file_name: String = entry.file_name().to_str().unwrap().to_owned();
		Resource
		{
			name: file_name.clone(),
			path: server_route.join(file_name),
			is_directory: entry.path().is_dir()
		}
	}
}