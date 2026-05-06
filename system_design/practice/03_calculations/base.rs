PRE
- MAU max unique MONTHLY ACTIVE USERS
- DAU max unique DAILY ACTIVE USERS
- 5_YEARS users total
- user content value per day

CALCULATIONS
- RPS request per second
- CONNECTIONS max in a moment
- CHANNAL_TRAFIC per seconds
- TOTAL MEMORY in 5 years
- COST

DAU/MAU
    Social Networks (Facebook, Instagram)	~50-60%
    Messengers (WhatsApp)	~70-80%
    Urs Shortener	~10-20%
    Image Hosting (как в задаче)	~20-30%
    Banking ~30-40%

DIAPASONS
- 1 modern server can handle 10-100k RPS, if bigger, provide for scaling
- max trafic
    1Gbps - cloud,
    10Gbps - twisted pair(in cluster),
    max 40Gbps - QSFP+ fiber optic
- 100+Gbps - infiniband(IB) very expencive, and used into super PCs

TRAFFIC PRICE
- 0.01-0.1$/Gb

UNITS
- Gb -gigabit - 1 million bits
- GB - gigabyte - 1 billion bytes, or 8 billion bits

RW
- 100/1 - basic ratio

MAX CONNECTIONS
- 10k

MAX AVAILABILITY
- CLOUD
    - 100k RPS - read text
    - 10k RPS - read db
    - 1k RPS - write db
- MY SERVER
    - 500k RPS - read text
    - 50k RPS - read db
    - 5k RPS - write db

STORAGE_TRAFFIC_AND_PRICE
- HDD 100-300 MBps, max 20TB <$500, 1TB = $30
- SSD 550 Mbps, max 8TB <$2000, 1TB = $300
- NVMe SSD 3-5 GBps, max 8TB <$4000, 1TB = $600
- RAM 50GBps, max 128Gb, about $1000 per unit, 1TB = $10000

PER_1_SERVER
- 1TB - RAM
- 50TB - SSD or 200TB - HDD

SPOILAGE
- 1% of all storages die per year

LATENCY
- go to L1 cashe - 0.5ns
- unguessed branching - 5ns
- go to L2 cashe - 7ns
- block unblock mutex - 100ns
- go to RAM - 100ns
- compress 1KB in zip - 10 000ns
- send 1KB through 1Gbps network - 10 000ns
- sequentially data read from RAM - 250 000ns/1MB
- inner cluster communication - 500 000ns
- relocation of HDD head - 10 000 000ns - 10ms
- sequentiolly data read from net - 10 000 000ns/1MB - 10ms/1MB
- sequentiolly data read from disk - 30 000 000ns/1MB - 30ms/1MB
- package through atlantic ocean - 150 000 000ns - 150ms

SCALE:
1 PB = 1000 TERABYTES
1 TERABYTE = 1000 GIGABYTES

Petabytes (PB)	1
Terabytes (TB)	1 024
Gigabytes (GB)  1 048 576
Megabytes (MB)	1 073 741 824
Kilobytes (KB)	1 099 511 627 776
