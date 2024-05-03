use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

pub struct ApiKey(String);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Extract the API key from the "x-api-key" header
        match request.headers().get_one("x-api-key") {
            Some(key) if key == "your_secret_api_key" => Outcome::Success(ApiKey(key.to_string())),
            Some(_) => Outcome::Failure((Status::Forbidden, ApiKeyError::Invalid)),
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
        }
    }
}


// #[get("/protected")]
// pub fn protected_route(_api_key: ApiKey) -> &'static str {
//     "This is a protected route."
// }

// #[macro_use] extern crate rocket;

// mod api_key_guard; // Import the module where ApiKey is defined
// use api_key_guard::ApiKey;

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![protected_route])
// }

// curl -H "x-api-key: your_secret_api_key" http://localhost:8000/protected

