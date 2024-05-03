use crate::Db;
use crate::schema::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use diesel::prelude::*;
use crate::models::MiningAuthorize;

#[post("/add_mining_authorize", format = "json", data = "<authorization>")]
pub async fn add_mining_authorize(db: Db, authorization: Json<MiningAuthorize>) -> Result<Status, Status> {
    db.run(move |conn| {
        use crate::schema::mining_authorize::dsl::*;
        diesel::insert_into(mining_authorize).values(&*authorization).execute(conn)
    }).await.map(|_| Status::Created).map_err(|_| Status::InternalServerError)
}
