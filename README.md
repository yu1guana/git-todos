# todos

A Git based Todos App.
This is a program written by Rust and provides similar functions to [z0al/git-todos](https://github.com/z0al/git-todos).
A diferrence from [z0al/git-todos](https://github.com/z0al/git-todos) is that this program correctly deal with non-ascii character such as Japanese.

## Installation

```sh
cd git-todos
cargo install --path .
```

## Usage

```sh
USAGE:
  git-todos <SUBCOMMAND>

OPTIONS:
  -h, --help       Print help information
  -V, --version    Print version information

SUBCOMMANDS:
  add       Add a new Todo
  edit      Edit a Todo
  finish    Fnish a Todo and commit stated changes
  help      Print this message or the help of the given subcommand(s)
  list      List available Todos
  remove    Remove existing Todo
  show      Show Todo details
```

## License

Copyright (c) 2023 Yuichi Ishida
Released under the MIT license
[https://opensource.org/licenses/mit-license.php](https://opensource.org/licenses/mit-license.php)
