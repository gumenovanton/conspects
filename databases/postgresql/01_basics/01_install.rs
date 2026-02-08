// run in docker for dev
docker run --rm -p 5432:5432 -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=1234 -e POSTGRES_DB=petstore -d postgres


# INSTALATION
// install
paru -S postgresql

// switch to postgres user
// do it in new terminal
sudo -iu postgres

// init postgres cluster
initdb --locale=C.UTF-8 --encoding=UTF8 -D /var/lib/postgres/data --data-checksums

// exit
exit

// run the postgres server
sudo systemctl enable --now postgresql

// create user
sudo -ip postgres
createuser --interactive

// create db
createdb --owner=ang demo

// exit
exit
