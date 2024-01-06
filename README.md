# PDF Viewer

PDF Viewer is a Rust UNIX cli tool for live reloading a pdf file, works if it's being compiled from
another program (LaTeX or Typst) then serving it to a browser via localhost:8080 (default port).
The UI is built with vanilla HTML, CSS and JavaScript.

The live reloading is done by polling, every 300ms (default duration) the program will check for
last modify time of the file then serves the data via a websocket to the browser.

## Usage

```sh
$ pdfviewer --help
Usage: pdfviewer [OPTIONS] <file>

Arguments:
  <file>  File to watch.

Options:
  -v, --verbose [<verbose>]  Verbose [default: 0] [possible values: 1, 0]
  -p, --port <port>          Port to serve pdf [default: 8080]
  -d, --duration <duration>  Polling duration in milliseconds [default: 300]
  -h, --help                 Print help
  -V, --version              Print version
```

## Installation

```sh
# to compile binary, creating a symlink to /usr/bin/pdfviewer,
# and to install static assets to $HOME/.pdfviewer
./installation.sh -i

# to remove binary from path
# and to delete the directory $HOME/.pdfviewer
./installation.sh -x
```
