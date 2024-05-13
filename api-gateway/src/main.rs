#[macro_use]
extern crate rocket;
use rocket::form::{Form, Strict};

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[derive(FromForm, Debug)]
struct Profile {
    username: String,
    password: String,
    withdraw_address: String,
}

#[post("/profile/register", data = "<profile>")]
fn register_profile(profile: Form<Profile>) {
    tracing::info!("{:?}", profile);

    
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
