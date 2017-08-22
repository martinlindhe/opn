# About

[![GoDoc](https://godoc.org/github.com/martinlindhe/open/open?status.svg)](https://godoc.org/github.com/martinlindhe/open/open)

`open` command to simulate the MacOS `open` command, targeting Linux and Windows


# Installation

    go get -u github.com/martinlindhe/open


# Windows usage

Open folder in Explorer:

    open .

Open url

    open https://google.com

Open file in default app:

    open main.go


# Linux usage

There usually is a different 'open' installed on the system.
To invoke it, we assume that you have exported the ~/go/bin folder to your
session's PATH before the system folders.

Something like this in your .bashrc:
```bash
export GOPATH="$HOME/go"
export PATH="$GOPATH/bin:$PATH"
```

## License

Under [MIT](LICENSE)
