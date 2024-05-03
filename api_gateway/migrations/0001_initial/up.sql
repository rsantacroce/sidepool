CREATE TABLE IF NOT EXISTS miner_profile (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    worker_name TEXT NOT NULL,
    password TEXT NOT NULL,
    payout_address TEXT NOT NULL,
    payment_scheme TEXT NOT NULL,
    is_activated BOOLEAN DEFAULT TRUE,
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS mining_authorize (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    host TEXT,
    channel_id INTEGER,
    worker_name TEXT,
    password TEXT,
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS mining_subscribe (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    host TEXT,
    channel_id INTEGER,
    user_agent TEXT,
    session_id TEXT,
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS submit_shares (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    host TEXT,
    channel_id INTEGER,
    sequence_number INTEGER,
    job_id INTEGER,
    nonce INTEGER,
    ntime INTEGER,
    version INTEGER,
    extranonce BLOB,
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS submit_solution (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    host TEXT,
    channel_id INTEGER,
    template_id INTEGER,
    version INTEGER,
    header_timestamp INTEGER,
    header_nonce INTEGER,
    coinbase_tx BLOB,
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS mining_set_difficulty (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    host TEXT,
    channel_id INTEGER,
    difficulty REAL,
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);


CREATE TABLE IF NOT EXISTS mining_notify (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    host TEXT,
    channel_id INTEGER,
    job_id TEXT,
    prevhash TEXT,
    coinb1 TEXT,
    coinb2 TEXT,
    merkle_branch TEXT,
    version TEXT,
    nbits TEXT,
    ntime TEXT,
    clean_jobs BOOLEAN,
    created_at INTEGER DEFAULT (strftime('%s', 'now'))  
);


CREATE TABLE IF NOT EXISTS hashrate_update_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    channel_id INTEGER,
    timestamp INTEGER,
    delta_time INTEGER,
    realized_shares_per_minute REAL,
    old_miner_hashrate REAL,
    new_miner_hashrate REAL,
    hashrate_change REAL,
    hashrate_change_percentage REAL,
    new_channel_nominal_hashrate REAL,
    shares_per_minute REAL,
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);


