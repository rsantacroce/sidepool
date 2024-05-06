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

### calculate hashrate on a block interval

```python
def calculateHashrateAndRewards(messages, start_time, end_time):
    jobs = {}  # Store job details from mining.notify
    submissions = []  # Store submissions from mining.submit
    rewards = {}  # Dictionary to store rewards per miner

    # Step 1: Collect data
    for message in messages:
        if message.type == 'mining.notify' and start_time <= message.timestamp <= end_time:
            jobs[message.job_id] = message
        elif message.type == 'mining.submit' and start_time <= message.timestamp <= end_time:
            submissions.append(message)

    # Step 2: Track jobs and shares
    for submission in submissions:
        job = jobs.get(submission.job_id)
        if job:
            difficulty = job.difficulty  # Assume difficulty is part of the job info
            time_taken = submission.timestamp - job.timestamp  # Simplification

            # Step 3: Calculate share difficulty
            hashrate = calculate_hashrate(difficulty, time_taken)

            # Step 4: Aggregate hashrates
            if submission.miner_id not in rewards:
                rewards[submission.miner_id] = {'hashrate': 0, 'reward': 0}
            rewards[submission.miner_id]['hashrate'] += hashrate

    # Step 5: Calculate rewards
    for block in found_blocks_during_period:
        total_pool_hashrate = sum(miner['hashrate'] for miner in rewards.values())
        block_reward = block.reward  # Total reward for the block

        for miner_id, data in rewards.items():
            miner_share = data['hashrate'] / total_pool_hashrate
            miner_reward = block_reward * miner_share
            rewards[miner_id]['reward'] += miner_reward

    return rewards

def calculate_hashrate(difficulty, time_taken):
    return (difficulty * 2**32) / time_taken

# Example usage:
messages = load_stratum_messages()  # Load or capture Stratum messages
start_time, end_time = get_period()  # Define the period for analysis
rewards = calculateHashrateAndRewards(messages, start_time, end_time)
print(rewards)
```