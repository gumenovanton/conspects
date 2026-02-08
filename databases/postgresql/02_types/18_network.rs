#### INET
// type for storing IP addresses
// 2 times less memory than text
CREATE TABLE inet_example (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    ip_address inet NOT NULL
);

INSERT INTO inet_example (ip_address) VALUES
('192.168.0.1/24'), // host address with subnet mask
('192.168.0.1'), // host address without subnet mask
('::1/128'), // IPv6 loopback address
('2001:db8::/32'), // IPv6 network
('2001:db8:85a3:8d3:1319:8a2e:370:7348'); // IPv6 host address

#### INET FUNCTIONS
SELECT ip_address
    hast(ip_address) AS host_only, // extracts host part
    masklen(ip_address) AS mask_length, // extracts the prefix length
    network(ip_address) AS network_only, // extracts the network part
    abbrev(ip_address) AS abbreviated_ip // abbreviates IPv6 address
FROM inet_example;

#### MAC ADDRESS
// type for storing MAC addresses
// 6 times less memory than text
CREATE TABLE macaddress_example (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    mac_address macaddr NOT NULL, // 6 bytes version
    mac_address macaddr8 NOT NULL // 8 bytes version
);

INSERT INTO macaddress_example (mac_address) VALUES
('00:11:22:33:44:55'), // MAC address
('00-11-22-33-44-55'), // MAC address with hyphens
('001122334455'); // MAC address without separators

#### MACADDRESS FUNCTIONS
SELECT mac_address
    macaddr8(mac_address) AS mac_address_8_bytes, // converts to 8-byte MAC address
    macaddr8_to_text(mac_address) AS mac_address_8_bytes_text, // converts to text
    macaddr_to_text(mac_address) AS mac_address_text // converts to text
FROM macaddress_example;
