// Home file sharing and storage server
// Backend implementation with Rust Rocket
// By Colby Reinhart
// 2-4-2023

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _
{
    rocket::build().mount("/", routes!
    [
        index,
    ])
}

#[get("/")]
async fn index() -> String
{
    "Hello world!".to_owned()
}
