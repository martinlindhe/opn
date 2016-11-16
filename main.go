package main

import (
	"fmt"
	"os"
	"os/exec"
)

func main() {

	args := []string{}
	if len(os.Args) < 2 {
		args = []string{"."}
	} else {
		args = os.Args[1:]
	}

	args = append([]string{"/c", "start"}, args...)

	cmd := exec.Command("cmd", args...)

	if err := cmd.Run(); err != nil {
		fmt.Println("error:", err)
		os.Exit(1)
	}
}
