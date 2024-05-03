use crate::schema::*;
use chrono::{NaiveDateTime, TimeZone};
use diesel::prelude::*;
use serde::{Deserialize, Serialize}; // This imports all table definitions from schema.rs

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = miner_profile)]
pub struct MinerProfile {
    pub id: Option<i32>,
    pub worker_name: String,
    pub password: String,
    pub payout_address: String,
    pub payment_scheme: String,
    pub is_activated: Option<bool>, // Update this field if it's nullable
    pub created_at: Option<i32>,    // Unix timestamp
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mining_authorize)]
pub struct MiningAuthorize {
    pub id: Option<i32>,
    pub host: String,
    pub channel_id: Option<i32>,
    pub worker_name: String,
    pub password: String,
    pub created_at: Option<i32>, // Unix timestamp
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mining_subscribe)]
pub struct MiningSubscribe {
    pub id: Option<i32>,
    pub host: String,
    pub channel_id: Option<i32>,
    pub user_agent: String,
    pub session_id: String,
    pub created_at: Option<i32>, // Unix timestamp
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = submit_shares)]
pub struct SubmitShares {
    pub id: Option<i32>,
    pub host: String,
    pub channel_id: Option<i32>,
    pub sequence_number: Option<i32>,
    pub job_id: Option<i32>,
    pub nonce: Option<i32>,
    pub ntime: Option<i32>,
    pub version: Option<i32>,
    pub extranonce: Option<Vec<u8>>,
    pub created_at: Option<i32>, // Unix timestamp
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = submit_solution)]
pub struct SubmitSolution {
    pub id: Option<i32>,
    pub host: String,
    pub channel_id: Option<i32>,
    pub template_id: Option<i32>,
    pub version: Option<i32>,
    pub header_timestamp: Option<i32>,
    pub header_nonce: Option<i32>,
    pub coinbase_tx: Option<Vec<u8>>,
    pub created_at: Option<i32>, // Unix timestamp
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mining_notify)]
pub struct MiningNotify {
    pub id: Option<i32>,
    pub host: String,
    pub channel_id: Option<i32>,
    pub job_id: String,
    pub prevhash: String,
    pub coinb1: String,
    pub coinb2: String,
    pub merkle_branch: String,
    pub version: String,
    pub nbits: String,
    pub ntime: String,
    pub clean_jobs: bool,
    pub created_at: Option<i32>, // Unix timestamp
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = hashrate_update_log)]
pub struct HashrateUpdateLog {
    pub id: Option<i32>,
    pub channel_id: Option<i32>,
    pub timestamp: Option<i32>,
    pub delta_time: Option<i32>,
    pub realized_shares_per_minute: Option<f32>,
    pub old_miner_hashrate: Option<f32>,
    pub new_miner_hashrate: Option<f32>,
    pub hashrate_change: Option<f32>,
    pub hashrate_change_percentage: Option<f32>,
    pub new_channel_nominal_hashrate: Option<f32>,
    pub shares_per_minute: Option<f32>,
    pub created_at: Option<i32>, // Unix timestamp
}
