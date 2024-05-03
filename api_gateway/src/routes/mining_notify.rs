use crate::models::MiningNotify;
use crate::Db;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

#[post("/add_mining_notify", format = "json", data = "<notify>")]
pub async fn add_mining_notify(db: Db, notify: Json<MiningNotify>) -> Result<Status, Status> {
    // log error if insert fails
    db.run(move |conn| {
        diesel::insert_into(crate::schema::mining_notify::table)
            .values(&notify.into_inner())
            .execute(conn)
    })
    .await
    .map(|_| Status::Created)
    .map_err(|x| {
        print!("Error inserting miner notify: {:?}", x);
        Status::InternalServerError
    })
}
