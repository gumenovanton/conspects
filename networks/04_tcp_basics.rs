#### TCP BASICS

>>>> ARP PACKAGE
// if a sender do not know a mac address off some destination host
// but know his ip
// it send an arp message to all hosts in the net
// like "who is the 183.383.03.199 host?"
// and that host answers with his mac addres to the sender host

// but do it every time is not effective
// and every host store all the mac addresses in the network in the arp cash
// to list cash
cat /proc/net/arp
// or with net-tools package with command arp
arp -a

// to scan and list all devices in the network i can use arp-scan package
// for example in wifi network
arp-scan --interface=wlan0 --localnet

#### IP CLASSES
// in past
// when you ask a static ip from your provider, they can give you three classes of static ips
- Class A // ip-s started from 0 to 126 and have subnet mask 255.0.0.0
// that means if they give you 120.0.0.0 subnet ip
// you can have any ips variants with 120 at first octet in your subnet
- Class B // ip-s started from 126 to 191 and have subnet mask 255.255.0.0
// that means if they give you 127.233.0.0 subnet ip
// you can have any ips variants with 127.233 at first two octets in your subnet
- Class C // ip-s started from 192 to 255 and have subnet mask 255.255.255.0
// that means if they give you 192.168.1.0 subnet ip
// you can have any ips variants with 192.168.1 at first tree octets in your subnet

#### CIDR - clasless inter-domain routing
// this is type of subneting when i can have subnet mask like
255.255.255.128
// and other

#### WHAT A MASK
// by mask host understand what he must do with request
// he compares his own ip with destination ip
// mask 255.255.255.0 means that
// if own ip and destination ip have equal bits in first 3 octets
// this request to host in the local network
// and host sends the arp package to local network
// if own ip and desination ip have not equal bits in first 3 octets
// host seng the request to default gateway(default route in linux), thats it the router

// when i do the network i gonna have three things for every host
- an ip of host
- a subnet mask
- a default gateway ip

#### DHCP
