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
