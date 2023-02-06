// Home file sharing and storage server
// Backend implementation with Rust Rocket
// By Colby Reinhart
// 2-4-2023

#[macro_use] extern crate rocket;

use home_file_share::{Config, Resource};
use rocket::{http::Status, State};
use rocket::response::content::RawJson;
use rocket::fs::NamedFile;
use std::fs::ReadDir;
use std::path::{Path, PathBuf};

#[launch]
fn rocket() -> _
{
	// Get config file
	let config: Config = Config::from_file(Path::new("Config.toml"));

	// Launch rocket
	rocket::build()
		.manage(config)
		.mount("/", routes!
		[
			index,
			get_folder_contents,
			get_file,
			create_folder
		])
}

//
// User routes
//

#[get("/")]
async fn index(config: &State<Config>) -> String
{
	config.server.storage_root_loc.to_str().unwrap().to_owned()
}

// Serve a file
#[get("/file/<filepath..>")]
async fn get_file(filepath: PathBuf, config: &State<Config>) -> Result<NamedFile, Status>
{
	// Construct actual filepath
	let full_path: PathBuf = config.server.storage_root_loc.join(&filepath);

	// Return 404 if it's a directory
	if full_path.is_dir()
	{
		Err(Status::BadRequest)
	}

	// Otherwise, return the requested file (if it exists)
	else
	{
		match NamedFile::open(full_path).await.ok()
		{
			Some(requested_file) => Ok(requested_file),
			None => Err(Status::NotFound)
		}
	}
}

//
// API routes
//

// Get the route and name of every file and folder in the given directory
#[get("/folder/<filepath..>")]
async fn get_folder_contents(filepath: PathBuf, config: &State<Config>)
-> Result<RawJson<String>, Status>
{
	let mut contents: Vec<Resource> = Vec::new();
	
	// Read the requested directory
	let directory: ReadDir =
	match std::fs::read_dir(config.server.storage_root_loc.join(&filepath))
	{
		Ok(result) => result,
		Err(error) =>
		{
			println!("{}", error);
			return Err(Status::InternalServerError)
		}
	};

	// Evaluate every entry in the directory
	for entry in directory.into_iter()
	{
		match entry
		{
			Ok(resource) =>
			{
				let server_path: PathBuf = PathBuf::from("/folder").join(&filepath);
				contents.push(Resource::from(resource, server_path));
			}
			Err(error) =>
			{
				println!("{}", error);
				return Err(Status::InternalServerError)
			}
		};
	}

	// Return all the data as a JSON
	Ok(RawJson(serde_json::to_string(&contents).unwrap()))
}

// Create a new directory on the server
#[get("/create-folder/<filepath..>")]
async fn create_folder(filepath: PathBuf, config: &State<Config>) -> Status
{
	// Construct the path and create the directory
	let full_path: PathBuf = config.server.storage_root_loc.join(&filepath);

	match std::fs::create_dir(full_path)
	{
		Ok(_) => Status::Ok,
		Err(error) =>
		{
			let error_message: String = error.to_string();

			// Does the file already exist?
			if error_message.contains("File exists")
			{
				Status::Conflict
			}

			// Is the filepath invalid?
			else if error_message.contains("No such file or directory")
			{
				Status::NotFound
			}

			// The only uncovered error type according to the docs is
			// invalid permissions, which qualifies as a 500
			else
			{
				println!("{}", error_message);
				Status::InternalServerError	
			}
		}
	}
}