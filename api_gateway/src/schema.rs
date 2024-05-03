// @generated automatically by Diesel CLI.

diesel::table! {
    hashrate_update_log (id) {
        id -> Nullable<Integer>,
        channel_id -> Nullable<Integer>,
        timestamp -> Nullable<Integer>,
        delta_time -> Nullable<Integer>,
        realized_shares_per_minute -> Nullable<Float>,
        old_miner_hashrate -> Nullable<Float>,
        new_miner_hashrate -> Nullable<Float>,
        hashrate_change -> Nullable<Float>,
        hashrate_change_percentage -> Nullable<Float>,
        new_channel_nominal_hashrate -> Nullable<Float>,
        shares_per_minute -> Nullable<Float>,
        created_at -> Nullable<Integer>,
    }
}

diesel::table! {
    miner_profile (id) {
        id -> Nullable<Integer>,
        worker_name -> Text,
        password -> Text,
        payout_address -> Text,
        payment_scheme -> Text,
        is_activated -> Nullable<Bool>,
        created_at -> Nullable<Integer>,
    }
}

diesel::table! {
    mining_authorize (id) {
        id -> Nullable<Integer>,
        host -> Nullable<Text>,
        channel_id -> Nullable<Integer>,
        worker_name -> Nullable<Text>,
        password -> Nullable<Text>,
        created_at -> Nullable<Integer>,
    }
}

diesel::table! {
    mining_notify (id) {
        id -> Nullable<Integer>,
        host -> Nullable<Text>,
        channel_id -> Nullable<Integer>,
        job_id -> Nullable<Text>,
        prevhash -> Nullable<Text>,
        coinb1 -> Nullable<Text>,
        coinb2 -> Nullable<Text>,
        merkle_branch -> Nullable<Text>,
        version -> Nullable<Text>,
        nbits -> Nullable<Text>,
        ntime -> Nullable<Text>,
        clean_jobs -> Nullable<Bool>,
        created_at -> Nullable<Integer>,
    }
}

diesel::table! {
    mining_set_difficulty (id) {
        id -> Nullable<Integer>,
        host -> Nullable<Text>,
        channel_id -> Nullable<Integer>,
        difficulty -> Nullable<Float>,
        created_at -> Nullable<Integer>,
    }
}

diesel::table! {
    mining_subscribe (id) {
        id -> Nullable<Integer>,
        host -> Nullable<Text>,
        channel_id -> Nullable<Integer>,
        user_agent -> Nullable<Text>,
        session_id -> Nullable<Text>,
        created_at -> Nullable<Integer>,
    }
}

diesel::table! {
    submit_shares (id) {
        id -> Nullable<Integer>,
        host -> Nullable<Text>,
        channel_id -> Nullable<Integer>,
        sequence_number -> Nullable<Integer>,
        job_id -> Nullable<Integer>,
        nonce -> Nullable<Integer>,
        ntime -> Nullable<Integer>,
        version -> Nullable<Integer>,
        extranonce -> Nullable<Binary>,
        created_at -> Nullable<Integer>,
    }
}

diesel::table! {
    submit_solution (id) {
        id -> Nullable<Integer>,
        host -> Nullable<Text>,
        channel_id -> Nullable<Integer>,
        template_id -> Nullable<Integer>,
        version -> Nullable<Integer>,
        header_timestamp -> Nullable<Integer>,
        header_nonce -> Nullable<Integer>,
        coinbase_tx -> Nullable<Binary>,
        created_at -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    hashrate_update_log,
    miner_profile,
    mining_authorize,
    mining_notify,
    mining_set_difficulty,
    mining_subscribe,
    submit_shares,
    submit_solution,
);
