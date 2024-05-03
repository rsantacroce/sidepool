#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket_sync_db_pools::database;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket::http::Method;

mod models;
mod routes;
mod schema;

use rocket::fs::FileServer;

#[database("sqlite_database")]
struct Db(diesel::SqliteConnection);

#[launch]
#[warn(unused_imports)]
fn rocket() -> _ {
    let cors = CorsOptions {
        // Specify the allowed origins
        allowed_origins: AllowedOrigins::all(),
        // Specify the allowed request methods
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        // Specify the allowed headers
        allowed_headers: rocket_cors::AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type"
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();

    rocket::build()
        .attach(Db::fairing())
        .attach(cors)
        .mount("/public", FileServer::from("static"))
        .mount(
            "/",
            routes![
                routes::add_miner_profile,
                routes::check_activation,
                routes::add_mining_authorize,
                routes::add_mining_subscribe,
                routes::add_submit_shares,
                routes::add_mining_notify,
                routes::add_hashrate_log,
            ],
        )
}
