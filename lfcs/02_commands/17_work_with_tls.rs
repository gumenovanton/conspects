## TLS
// tls - transport layer security

// to create a tls sertificate i must use openssl command
// openssl have many manual pages

## to output all openssl manual pages
man openssl

// the most used command is
openssl req
// and
openssl x509

// to get help of them
// and the better way is to go to example section
man openssl req
man openssl x509

# HOW TO CREATE A CERTIFICATE
// if i want to whole world to trust my certificate:
// step one - create a sertificate request
// step two - send it to Certificate Authority and they will check all data in it
// step three - get a certificate from Certificate Authority
// step four - install certificate

// if i want to use it internally in my network:
// step one - create a sertificate request
// step two - create a self-signed certificate

## how to create a certificate request
openssl req -newkey rsa:2048 -keyout key.pem -out req.pem
// -newkey rsa:2048 - create a new private key with rsa algorithm and size 2048 bits
// -keyout key.pem - output file for private key
// -out req.pem - output file for certificate request

// press enter and type your any password
// then input all information that program will want like country,
// state, locality, organization, organizational unit, common name(website), email address etc.

## how to create a self-signed certificate
openssl req -x509  -newkey rsa:2048 -days 365 -keyout private.key -out mycertificate.crt
// -x509 - create a self-signed certificate
// -newkey rsa:2048 - create a new private key with rsa algorithm and size 2048 bits
// -days 365 - how long certificate will be valid
// -keyout private.key - output file for private key
// -out mycertificate.crt - output file for certificate

// press enter and type your any password
// then input all information that program will want like country,
// state, locality, organization, organizational unit, common name(website), email address etc.

## how to decode sertificate
openssl x509 -in mycertificate.crt -text
