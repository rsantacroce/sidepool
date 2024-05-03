use crate::models::MinerProfile;
use crate::Db;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

#[post("/add_miner_profile", format = "json", data = "<profile>")]
pub async fn add_miner_profile(db: Db, profile: Json<MinerProfile>) -> Result<Status, Status> {

    // log error if insert fails
    db.run(move |conn| {
        diesel::insert_into(crate::schema::miner_profile::table)
            .values(&profile.into_inner())
            .execute(conn)
    })
    .await
    .map(|_| Status::Created)
    .map_err(|x| { 
        print!("Error inserting miner profile: {:?}", x);
        Status::InternalServerError
    })
    
}

#[get("/check_activation/<worker_name>/<password>")]
pub async fn check_activation(
    db: Db,
    worker_name: String,
    password: String,
) -> Result<Json<Option<bool>>, Status> {
    db.run(move |conn| {
        use crate::schema::miner_profile::dsl::*;
        miner_profile
            .filter(worker_name.eq(worker_name).and(password.eq(password)))
            .select(is_activated)
            .first::<Option<bool>>(conn)
    })
    .await
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}
