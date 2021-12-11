package main

import (
	"fmt"

	"github.com/suborbital/reactr/rt"
	"github.com/suborbital/reactr/rwasm"
)

func main() {
	r := rt.New()
	doWasm := r.Register("hello-quickjs", rwasm.NewRunner("./rs_embed_js.wasm"))

	code := 
		"let h = 'hello';" +
		"let w = 'wasmedge';" +
		"`${h} ${w}`; // eval_return"
	res, err := doWasm([]byte(code)).Then()
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Println(string(res.([]byte)))
}