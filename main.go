package main

import (
	"fmt"
	"os"
	"os/exec"
)

func main() {

	if len(os.Args) < 2 {
		fmt.Println("missing parameters")
		os.Exit(1)
	}

	args := []string{"/c", "start"}
	args = append(args, os.Args[1:]...)

	cmd := exec.Command("cmd", args...)

	if err := cmd.Run(); err != nil {
		fmt.Println("error:", err)
		os.Exit(1)
	}
}
