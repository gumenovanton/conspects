//>> goose
// utill to realize migrations

//>> how to use
//<< step 1: install
go install github.com/pressly/goose/v3/cmd/goose@latest

//<< step 2: create .env and put next vars into it
// this is db config
DB_NAME=users_db
DB_USER=ang
DB_PASSWORD=1234
DB_HOST=db
DB_PORT=5432

// this is vars needed by goose to generate migration files
GOOSE_DRIVER=postgres
GOOSE_DBSTRING=postgresql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
GOOSE_MIGRATION_DIR= ./internal/modules/repository/migrations

//<< step 3: generate migration files
// numeric naming
goose -s create filename sql // creates 00001_filename.sql

// date naming
goose create filename sql // creates 20250413081836_filename.sql

//<< step 4: up or down migrations
// all up
goose up
// all down
goose down
// up to specific
goose up-to 00003
// down to specific
goose down-to 00003
// up by one
goose up-by-one
// down by one
goose down-be-one

//>> running migrations in code
//<< by embided file
//go:embed migrations/*.sql
var migrations embed.FS
goose.SetBaseFS(migrations)

err := goose.SetDialect("postgres")

db := stdlib.OpenDBFromPool(r.pool)

err := goose.Up(db, "migrations")

err := db.Close()

//<< in code migrations 
package migration

import (
	"database/sql"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigration(Up, Down)
}

func Up(tx *sql.Tx) error {
	_, err := tx.Exec("UPDATE users SET username='admin' WHERE username='root';")
	if err != nil {
		return err
	}
	return nil
}

func Down(tx *sql.Tx) error {
	_, err := tx.Exec("UPDATE users SET username='root' WHERE username='admin';")
	if err != nil {
		return err
	}
	return nil
}