package open

import (
	"fmt"
	"os"
	"os/exec"
)

func OpenPath(args []string) {
	args = append([]string{"/c", "start"}, args...)

	cmd := exec.Command("cmd", args...)

	if err := cmd.Run(); err != nil {
		fmt.Println("error:", err)
		os.Exit(1)
	}
}
