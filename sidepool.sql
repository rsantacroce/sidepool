-- Create the mining_subscribe table
CREATE TABLE mining_subscribe (
    id SERIAL PRIMARY KEY,
    host TEXT NOT NULL,
    user_agent TEXT,
    session_id TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create an index on the 'host' column for better query performance
CREATE INDEX idx_mining_subscribe_host ON mining_subscribe (host);

-- Create the mining_authorize table
CREATE TABLE mining_authorize (
    id SERIAL PRIMARY KEY,
    host TEXT NOT NULL,
    port INTEGER NOT NULL,
    worker_name TEXT NOT NULL,
    password TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create an index on the 'host' column for better query performance
CREATE INDEX idx_mining_authorize_host ON mining_authorize (host);

-- Create an index on the 'worker_name' column for better query performance
CREATE INDEX idx_mining_authorize_worker_name ON mining_authorize (worker_name);
