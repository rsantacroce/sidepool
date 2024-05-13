# sidepool tasks list 

### setup, bootstrap and cpumining

    DOING - store shares and relevante information from mining
    - improve the default shares calculation
    - improve user authentication for the pool with authorization and authentication

    - DONE setup bitcoin signet, pool (range distribution and template provider), translator proxy
    - DONE integration with cpuminer with user (handling messages)
    - DONE implemented dashboard, logs managt, auditability and instrumentation
    - DONE devops to deploy pool and dev env


### pool user interface

    DOING - public dashboard with pool information 
    - dashboard 
        - shares contributed 
        - balance 
        - withdraw

    DONE - create webapp with user profile (can be public with username provided)
        DONE - login and register
        DONE - update profile with payment address
        
    

### bitcoin and payment integration

    - integrate with bmm messages
    - integrate thunder deposit address as username 
    - deposit bitcoin from pool wallet to thunder sidechain
    - distribute the payment using thunder deposit address from pool thunder wallet 


## relevante information
    - template id / job id 
    - prev_block (information after it's been mined)
    - mining.authorize and subscription unique by host:port
    - submit shares
    - solution submit (when accepted and when rejected)

## notes on random code 

### useful sql queries

```sql
/*
-- list side by side events in their proper sequence kind of flatten */
select 
	payload->>'id' as payload_id,
	payload->>'method' as method, 
	payload->>'result' as result,
	id, 
	host,
	created_at
from
	mining_events me
order by
	created_at ,
	payload->>'id'

/*
 * extract each field from a notify message
*/
select
	json_array_element_text(payload->'params',
	0) as job_id,
	json_array_element_text(payload->'params',
	1) as prev_hash,
	json_array_element_text(payload->'params',
	2) as coin_base1,
	json_array_element_text(payload->'params',
	3) as coin_base2,
	json_array_element_text(payload->'params',
	4) as merkle_branch,
	json_array_element_text(payload->'params',
	5) as version,
	json_array_element_text(payload->'params',
	6) as bits,
	json_array_element_text(payload->'params',
	7) as time,
	json_array_element_text(payload->'params',
	8) as clean_jobs,
	host,
	created_at,
	id
from
	mining_events me
where
	payload->>'method' = 'mining.notify'
 */


/* list all shares contributions */
select 
		json_array_element_text(payload->'params',
	0) as username,
		json_array_element_text(payload->'params',
	1) as job_id,
		json_array_element_text(payload->'params',
	2) as extranonce2,
		json_array_element_text(payload->'params',
	3) as time,
		json_array_element_text(payload->'params',
	4) as nonce,
	host,
	created_at,
	id
from
	mining_events me
where
	payload->>'method' = 'mining.submit'
*/

/* calculate the hashrate for each users hourly */

WITH relevant_difficulties AS (
    SELECT
        created_at AS timestamp,
        (payload->'params'->>0)::NUMERIC AS difficulty
    FROM
        mining_events
    WHERE
        payload->> 'method' = 'mining.set_difficulty'
),
ordered_shares AS (
    SELECT
        payload->'params'->>0 AS miner,
        payload->'params'->>1 AS job_id,
        created_at AS timestamp,
        COALESCE(rd.difficulty, LAG(rd.difficulty) OVER (PARTITION BY payload->'params'->>0 ORDER BY created_at)) AS difficulty,
        LAG(created_at) OVER (PARTITION BY payload->'params'->>0 ORDER BY created_at) AS previous_timestamp
    FROM
        mining_events mr
    LEFT JOIN
        relevant_difficulties rd ON mr.created_at >= rd.timestamp
    WHERE
        payload->> 'method' = 'mining.submit'
),
hashrate_calculations AS (
    SELECT
        miner,
        job_id,
        timestamp,
        difficulty,
        EXTRACT(EPOCH FROM (timestamp - previous_timestamp)) AS time_taken -- Time taken in seconds
    FROM
        ordered_shares
    WHERE
        previous_timestamp IS NOT NULL -- To avoid the first record problem
),
hashrates AS (
    SELECT
        miner,
        timestamp,
        difficulty,
        CASE
            WHEN time_taken = 0 THEN NULL -- Avoid division by zero
            ELSE (difficulty * 2^32) / time_taken
        END AS hashrate
    FROM
        hashrate_calculations
)
SELECT
    miner,
    date_trunc('hour', timestamp) AS hour,
    AVG(hashrate) AS average_hashrate -- Calculate the average hashrate per miner per hour
FROM
    hashrates
GROUP BY
    miner, date_trunc('hour', timestamp);


/* total contribution to the pool by user */
WITH relevant_difficulties AS (
    SELECT
        created_at AS timestamp,
        (payload->'params'->>0)::NUMERIC AS difficulty
    FROM
        mining_events
    WHERE
        payload->> 'method' = 'mining.set_difficulty'
),
ordered_shares AS (
    SELECT
        payload->'params'->>0 AS miner,
        payload->'params'->>1 AS job_id,
        created_at AS timestamp,
        COALESCE(rd.difficulty, LAG(rd.difficulty) OVER (PARTITION BY payload->'params'->>0 ORDER BY created_at)) AS difficulty,
        LAG(created_at) OVER (PARTITION BY payload->'params'->>0 ORDER BY created_at) AS previous_timestamp
    FROM
        mining_events mr
    LEFT JOIN
        relevant_difficulties rd ON mr.created_at >= rd.timestamp
    WHERE
        payload->> 'method' = 'mining.submit'
),
hashrate_calculations AS (
    SELECT
        miner,
        job_id,
        timestamp,
        difficulty,
        EXTRACT(EPOCH FROM (timestamp - previous_timestamp)) AS time_taken -- Time taken in seconds
    FROM
        ordered_shares
    WHERE
        previous_timestamp IS NOT NULL -- To avoid the first record problem
),
hashrates AS (
    SELECT
        miner,
        timestamp,
        difficulty,
        CASE
            WHEN time_taken = 0 THEN NULL -- Avoid division by zero
            ELSE (difficulty * 2^32) / time_taken
        END AS hashrate
    FROM
        hashrate_calculations
),
hourly_totals AS (
    SELECT
        date_trunc('hour', timestamp) AS hour_,
        SUM(hashrate) AS total_hashrate
    FROM
        hashrates
    GROUP BY
        hour_
),
hourly_miner_hashrate AS (
    SELECT
        miner,
        date_trunc('hour', timestamp) AS hour_,
        AVG(hashrate) AS average_hashrate
    FROM
        hashrates
    GROUP BY
        miner, hour_
)
SELECT
    hmh.miner,
    hmh.hour_,
    hmh.average_hashrate,
    hmh.average_hashrate / ht.total_hashrate * 100 AS pool_percentage
FROM
    hourly_miner_hashrate hmh
JOIN
    hourly_totals ht ON hmh.hour_ = ht.hour_
ORDER BY
    hmh.hour_, hmh.miner;
```