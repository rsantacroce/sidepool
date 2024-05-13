# Sidepool Dev Guide

In order to facilitate things we are running a customized version of bitcoin signet docker container, as well as some other tools, such as postgres and grafana loki.

Make sure you have docker running:

```shell
docker ps
```

## Repositories

### Sidepool tools and api-gateway

Designed to be a monorepo with all dependencies this should be your starting point

https://github.com/rsantacroce/sidepool


### Stratum V2 SRI customizations

https://github.com/rsantacroce/stratum-sidepool/

### Bitcoin Signet with SV2 Template Provider Protocol

https://github.com/rsantacroce/bitcoin_signet/


## Setting up the env

1. Clone the sidepool repository 

```shell
git clone https://github.com/rsantacroce/sidepool.git
```

2. Clone the stratum-sidepool repository

```shell
git clone https://github.com/rsantacroce/stratum-sidepool.git
```

3. Run bitcoin signet 

This will download the bitcoin_signet repository and create a local docker image

```shell
cd sidepool
./run_bitcoin_signet.sh
```

It's important to wait until you have at least 16 blocks on signet you can check it using:

```shell
docker logs -f bitcoin-signet-instance 
```

4. Run PostgreSQL

If you dont have on your machine or are not using another remote instance, this image will run a pgsql database and also a webadmin.

```shell
cd sidepool
docker-compose -f postgres.yml up -d
```

After the images and instances are created you need to create the database, log in the admin using the following credentials:

http://localhost:8080 

|          |          |
|----------|----------|
| username | postgres |
| password | password |
| dbname   | postgres |


Create a database with name "sidepool", select it and execute the content of [sidepool.sql]() as a SQL Statement on the interface.

5. Starting the Pool 

The pool needs to connect to the Bitcoin Signet Template Provider

```shell
cd sidepool
./run_roles.sh
# select option 1
```

6. Starting the Proxy

This is the main component that communicate with the miners and the database as well as the pool.

```shell
cd sidepool
./run_roles.sh
# select option 1
```

7. CPU Mining

Download the cpuminer from [https://sourceforge.net/projects/cpuminer/files/]

```shell
./minerd -a sha256d -o stratum+tcp://localhost:34255 -q -D -P -u YOUR_USERNAME_OR_THUNDER_ADDRESS
```

8. Extract shares details

Execute the following query on the [http://localhost:8080](pgadmin)

```sql
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


