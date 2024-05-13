# sidepool

A open source naive implementation of a mining pool using SRI and enabling multiple payment schemes and payouts such as L2 and L1.

## Objective

This is far from being production ready, is mostly tested on signet to simulate hashrate difficulty adj.

There is some instrumentations needed in order to create a post billing kind of mining pool.

DONE - instrument all the messages in order to have a proper dashboard of events
WIP - create a proper webui / rest api to maintain the basic of a mining pool:
    - user login and profile
    - dashboards with stats both profile and public
    - withdraw panels and contribution history
- create a customization layer for payment scheme e payment delivery

## References

- [API Gateway](api-gateway)
- [Tasks](tasks.md)
- [Dev docs](DEV.md)
- [DB](sidepool.sql)
- [Default Configuration](config/)

## notes 
THANK YOU ! 

The whole code is strongly based on the increrible work from the [Stratum SV 2 Team](https://github.com/stratum-mining/stratum).

