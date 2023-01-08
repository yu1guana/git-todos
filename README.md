# git-todos

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
Usage: git todos <COMMAND>

Commands:
  list    List available Todos
  show    Show Todo details
  add     Add a new Todo
  edit    Edit a Todo
  finish  Fnish a Todo and commit stated changes
  remove  Remove existing Todo
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## License

Copyright (c) 2023 Yuichi Ishida
Released under the MIT license
[https://opensource.org/licenses/mit-license.php](https://opensource.org/licenses/mit-license.php)
