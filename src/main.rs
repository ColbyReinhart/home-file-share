// Home file sharing and storage server
// Backend implementation with Rust Rocket
// By Colby Reinhart
// 2-4-2023

#[macro_use] extern crate rocket;

use home_file_share::Config;
use rocket::State;
use std::path::Path;

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
		])
}

#[get("/")]
async fn index(config: &State<Config>) -> String
{
	config.server.storage_root_loc.to_str().unwrap().to_owned()
}
