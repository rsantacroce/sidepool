# running sidepool signet env

mainly testing on signet so we highly recomend using the sv2 enabled bitcoin core 


## services

There is two main groups of services:

1. Infra Structure
2. Pool

### bitcoin signet

using docker and docker-compose:

    ./sidepool/run_bitcoin_signet.sh

### postgresql with admin 

    docker-compose -f postgres.yml up -d 

### grafana, loki and promtail

    docker-compose -f loki.yml up -d 

### pool

    ./sidepool/run_roles.sh 1
    ./sidepool/run_roles.sh 2
    ./minerd -a sha256d -o stratum+tcp://localhost:34255 -q -D -P -u nakamoto2

