# About

`opn` takes an argument of a file, path or a URL, and opens the registered filetype handler, file explorer or web browser accordingly.

The `opn` command simulates the MacOS `open` command, targeting Linux and Windows.


# Installation

    go install github.com/martinlindhe/opn@latest


# Usage

Open folder in the file explorer:

    opn .

Open url in default browser:

    opn https://google.com

Open file in default app:

    opn main.go


## License

Under [MIT](LICENSE)
