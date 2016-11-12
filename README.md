# maze-rs
Maze for Programmers (Rust)

## Status

[![Build Status](https://travis-ci.org/korczis/maze-rs.svg?branch=master)](https://travis-ci.org/korczis/maze-rs)

## Usage

```
$ ./target/debug/maze -h
Maze Generator 0.1.0
Tomas Korcak <korczis@gmail.com>

USAGE:
    maze [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode

OPTIONS:
    -a, --algorithm <algorithm>    Algorithm to use [default: binary]  [values: binary, sidewinder]
    -y, --height <height>          Height of Maze [default: 5]
    -x, --width <width>            Width of Maze [default: 5]
```

## Generator Algoritms

- [x] Binary
- [x] Sidewinder

## Output Formats

- [x] ASCII Art
- [ ] PNG
- [ ] JSON

## Benchmark

```
$ time ./target/release/maze -x 1000 -y 1000 > out.txt

real	0m1.423s
user	0m1.149s
sys	0m0.241s
```

## Example

```
$ ./target/debug/maze -x 10 -y 10

+---+---+---+---+---+---+---+---+---+---+
|   |   |               |               |
+   +   +---+---+---+   +---+---+---+   +
|   |   |   |               |   |       |
+   +   +   +---+---+---+   +   +---+   +
|   |   |                           |   |
+   +   +---+---+---+---+---+---+   +   +
|       |                       |   |   |
+---+   +---+---+---+---+---+   +   +   +
|   |       |   |   |       |       |   |
+   +---+   +   +   +---+   +---+   +   +
|               |   |   |               |
+---+---+---+   +   +   +---+---+---+   +
|   |           |   |       |           |
+   +---+---+   +   +---+   +---+---+   +
|                               |       |
+---+---+---+---+---+---+---+   +---+   +
|   |   |       |       |   |   |   |   |
+   +   +---+   +---+   +   +   +   +   +
|                                       |
+---+---+---+---+---+---+---+---+---+---+
```