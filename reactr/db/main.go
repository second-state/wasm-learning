package main

import (
	"fmt"
	"os"

	"github.com/suborbital/reactr/rcap"
	"github.com/suborbital/reactr/rt"
	"github.com/suborbital/reactr/rwasm"
	"github.com/suborbital/vektor/vlog"
)

func main() {
	dbConnString, exists := os.LookupEnv("REACTR_DB_CONN_STRING")
	if !exists {
		fmt.Println("skipping as conn string env var not set")
		return
	}

	q1 := rcap.Query{
		Type:     rcap.QueryTypeInsert,
		Name:     "PGInsertUser",
		VarCount: 2,
		Query: `
		INSERT INTO users (uuid, email, created_at, state, identifier)
		VALUES ($1, $2, NOW(), 'A', 12345)`,
	}

	q2 := rcap.Query{
		Type:     rcap.QueryTypeSelect,
		Name:     "PGSelectUserWithUUID",
		VarCount: 1,
		Query: `
		SELECT * FROM users
		WHERE uuid = $1`,
	}

	q3 := rcap.Query{
		Type:     rcap.QueryTypeUpdate,
		Name:     "PGUpdateUserWithUUID",
		VarCount: 1,
		Query: `
		UPDATE users SET state='B' WHERE uuid = $1`,
	}

	q4 := rcap.Query{
		Type:     rcap.QueryTypeDelete,
		Name:     "PGDeleteUserWithUUID",
		VarCount: 1,
		Query: `
		DELETE FROM users WHERE uuid = $1`,
	}

	config := rcap.DefaultConfigWithDB(vlog.Default(), rcap.DBTypePostgres, dbConnString, []rcap.Query{q1, q2, q3, q4})

	r, err := rt.NewWithConfig(config)
	if err != nil {
		fmt.Println(err)
		return
	}

	doWasm := r.Register("rs-db", rwasm.NewRunner("./rs_db.wasm"))

	res, err := doWasm(nil).Then()
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Println(string(res.([]byte)))
}
