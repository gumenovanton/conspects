BASE
- mau, dau, increase content through the day, 5 year, R/W
- 10 connections, 1Gbps channel, max $0.1/GB trafic
- 100k RPS text, 10k RPS read db, 1k RPS write db
- storage
    HDD 300MB/s, $30/TB
    SSD 550BM/s, $300/TB
    SSD 5GB/s, $600/TB
    RAM 50GB/s, $10000/TB
        - 1 server = 1TB RAM, 50TB SSD or 200TB HDD

ALGORITHM
    - think about
        - MAU,
        - DAU,
        - Read Requests per user
        - Write Requests per user
        - Request Size
    - R/W per month
    - R/W RPS
    - trafic
        - R Trafic
        - W Trafic
    - calculate memory 5 year total

- think about MAU, DAU, and content through the day, and all in 5 years
    - 10M users at all
    - 1M dayly users
    - every user makes 3 write request per day
    - every request ~ 1KB

- calculate R/W request per month
    1M dayly * 30days * 3w.req ~ 100M write requests per month.
    100M*100(read/write=100/1) 10B read requests per month.

- calculate R/W RPS (2592000 sec in month)
    100M/(30*86400sec/day)=100M/2592k =39RPS on write
    39*100 =3900 RPS on read

- calculate max trafic per sec
    4k RPS * 1KB = 4000KBs / 128 = 32Mbs

- calculate total 5 year memory
    100M*5*12 = 6B records
    6B * 1KB = 6B KB = 6TB total

- calculate price
    40RPS - one server
    6TB - one server
    6TB * $0.1GB = $4915.2



- 50m MAU
- 15m DAU
- R/W = 20/3
- 2MB per request after save
