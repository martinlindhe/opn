package main

import (
	"os"

	"github.com/martinlindhe/opn/open"
)

func main() {

	args := []string{}
	if len(os.Args) < 2 {
		args = []string{"."}
	} else {
		args = os.Args[1:]
	}

	open.OpenPath(args)
}
