package main

import (
	"fmt"

	"github.com/suborbital/reactr/rt"
	"github.com/suborbital/reactr/rwasm"
)

func main() {
	runBundle()
	runGroup()
}

func runBundle() {
	r := rt.New()
	doWasm := r.Register("hello-echo", rwasm.NewRunner("./hello_echo.wasm"))

	res, err := doWasm([]byte("wasmWorker!")).Then()
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Println(string(res.([]byte)))
}

func runGroup() {
	r := rt.New()

	doWasm := r.Register("hello-echo", rwasm.NewRunner("./hello_echo.wasm"))

	grp := rt.NewGroup()
	for i := 0; i < 100000; i++ {
		grp.Add(doWasm([]byte(fmt.Sprintf("world %d", i))))
	}

	if err := grp.Wait(); err != nil {
		fmt.Println(err)
	}
}