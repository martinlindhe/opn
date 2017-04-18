package open

import (
	"fmt"
	"os"
	"os/exec"
)

func OpenPath(args []string) {
	cmd := exec.Command("xdg-open", args...)

	if err := cmd.Run(); err != nil {
		fmt.Println("error:", err)
		os.Exit(1)
	}
}
