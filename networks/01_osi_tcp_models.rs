#### OSI and TCP/IP
// OSI has 7 levels     TCP/IP has 4 levels         From BOTTOM to UP flow                                                                                                                                                          From UP to BOTTOM flow
7 - application         1 - application             // get the data, represent it to the app readable form and lift it to app                                                                                                       // get the data from the app and converts it to bytes
6 - presentation
5 - session

4 - transport           2 - transport               // get a frames, assemble them into the full data message by port, and lift it up, too app port                                                                                 // disassemble the data to frames and add ports

3 - network             3 - internet                // get a frame,  checks the ips, cut off the ip headers and lift the frame up                                                                                                   // add ips to the frame

2 - data link           4 - network interface       // get a frame, checks the mac address, if mac address for him, cut off the mac addresses and the trailer with check summs, store the sender macaddress and lift the frame up      // add macs and trailers checksums to frames and send it
1 - phisical

#### MAC ADDRESS DETALES
// ff:ff:ff:ff:ff:ff - the broadcast macadress, that means all devices in the net never get rid of it

// all macs are hardcoded to device and it can't be changed or erased
// first tree symbols - identify the factory
// last three symbols - identify the device

#### IP
// all devices has ips
// if one send a broadcast frame, hub will send it to all devices in its net, and switch will work like a hub
// but what if there are thousands devices in the net
// and every one send message to all
// it will be like DOS atack
// noone can work because the net will be overloaded

// because of it, ip addresses was created

#### PROS OF IP ADDRESSES
// ip address - identify the network, and the device in this network
// when a device send a frame where the destination ip not in its own network
// the frame will be send to the default geteway, and this is the router
// destination mac addless in this case will be the router mac address

#### WHAT HAPPENS WHEN FRAME RICH THE ROUTER


#### DIFFERENCE
// tcp - connection oriented, with check that the frame is received
// udp - connectionless, without check thet the frame is received
