package main

import (
	"fmt"
	"io/ioutil"
	"os"

	"github.com/suborbital/reactr/rt"
	"github.com/suborbital/reactr/rwasm"
)

func main() {
	r := rt.New()
	doWasm := r.Register("hello-quickjs", rwasm.NewRunner("./rs_embed_js.wasm"))

	code, err := ioutil.ReadFile(os.Args[1])
	if err != nil {
		fmt.Print(err)
	}
	res, err := doWasm(code).Then()
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Println(string(res.([]byte)))
}
