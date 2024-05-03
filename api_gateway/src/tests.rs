#[cfg(test)]
mod test {
    use rocket::local::asynchronous::Client;
    use rocket::http::{Status, ContentType};
    use serde_json::json;

    #[rocket::async_test]
    async fn test_add_miner_profile() {
        let rocket = rocket::build()
            .attach(super::Db::fairing())
            .mount("/", routes![super::routes::add_miner_profile]);
        let client = Client::tracked(rocket).await.expect("valid rocket instance");
        let response = client.post("/add_miner_profile")
            .header(ContentType::JSON)
            .body(json!({
                "worker_name": "miner123",
                "password": "securepass",
                "payout_address": "1Aa2Bb3Cc4Dd5Ee6Ff",
                "payment_scheme": "PPLNS",
                "is_activated": true
            }).to_string())
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Created);
    }

    #[rocket::async_test]
    async fn test_add_mining_authorize() {
        let rocket = rocket::build()
            .attach(super::Db::fairing())
            .mount("/", routes![super::routes::add_mining_authorize]);
        let client = Client::tracked(rocket).await.expect("valid rocket instance");
        let response = client.post("/add_mining_authorize")
            .header(ContentType::JSON)
            .body(json!({
                "host": "127.0.0.1",
                "worker_name": "miner123",
                "password": "securepass"
            }).to_string())
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Created);
    }

    #[rocket::async_test]
    async fn test_add_mining_subscribe() {
        let rocket = rocket::build()
            .attach(super::Db::fairing())
            .mount("/", routes![super::routes::add_mining_subscribe]);
        let client = Client::tracked(rocket).await.expect("valid rocket instance");
        let response = client.post("/add_mining_subscribe")
            .header(ContentType::JSON)
            .body(json!({
                "host": "127.0.0.1",
                "user_agent": "test-agent",
                "session_id": "session123"
            }).to_string())
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Created);
    }
}
