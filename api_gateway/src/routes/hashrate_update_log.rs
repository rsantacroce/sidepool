use crate::models::HashrateUpdateLog;
use crate::Db;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

#[post("/add_hashrate_log", format = "json", data = "<hashrate_log>")]
pub async fn add_hashrate_log(
    db: Db,
    hashrate_log: Json<HashrateUpdateLog>,
) -> Result<Status, Status> {
    db.run(move |conn| {
        diesel::insert_into(crate::schema::hashrate_update_log::table)
            .values(&hashrate_log.into_inner())
            .execute(conn)
    })
    .await
    .map(|_| Status::Created)
    .map_err(|x| {
        print!("Error inserting miner profile: {:?}", x);
        Status::InternalServerError
    })
}
