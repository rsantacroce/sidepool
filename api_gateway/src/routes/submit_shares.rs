use crate::Db;

use crate::models::SubmitShares;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

#[post("/add_submit_shares", format = "json", data = "<shares>")]
pub async fn add_submit_shares(db: Db, shares: Json<SubmitShares>) -> Result<Status, Status> {
    db.run(move |conn| {
        use crate::schema::submit_shares::dsl::*;
        diesel::insert_into(submit_shares)
            .values(&*shares)
            .execute(conn)
    })
    .await
    .map(|_| Status::Created)
    .map_err(|_| Status::InternalServerError)
}
