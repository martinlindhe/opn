# About

[![GoDoc](https://godoc.org/github.com/martinlindhe/open/open?status.svg)](https://godoc.org/github.com/martinlindhe/open/open)

`open` command to simulate the MacOS `open` command, targeting Linux and Windows


# Installation

    go get -u github.com/martinlindhe/open


# Usage

Open folder in Explorer:

    open .

Open url

    open https://google.com

Open file in default app:

    open main.go


# Linux usage

There usually is a different 'open' installed on the system.
To invoke it, we assume that you have your exported to your
session's PATH before the system folders.

Something like this in your .bashrc:
```bash
export GOPATH="$HOME/dev/go"
export GOBIN="$GOPATH/bin"
export PATH="$GOBIN:$PATH"
```

## License

Under [MIT](LICENSE)
