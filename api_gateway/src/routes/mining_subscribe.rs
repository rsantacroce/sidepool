use crate::Db;

use rocket::http::Status;
use rocket::serde::json::Json;
use diesel::prelude::*;
use crate::models::MiningSubscribe;

#[post("/add_mining_subscribe", format = "json", data = "<subscription>")]
pub async fn add_mining_subscribe(db: Db, subscription: Json<MiningSubscribe>) -> Result<Status, Status> {
    db.run(move |conn| {
        use crate::schema::mining_subscribe::dsl::*;
        diesel::insert_into(mining_subscribe).values(&*subscription).execute(conn)
    }).await.map(|_| Status::Created).map_err(|_| Status::InternalServerError)
}
