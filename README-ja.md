# git-todos

A Git based Todos App.
これはRustで書かれたプログラムで、[z0al/git-todos](https://github.com/z0al/git-todos)と似たような機能を提供します。
[z0al/git-todos](https://github.com/z0al/git-todos)との違いは非アスキー文字を扱えることです。

## インストール方法

```sh
cd git-todos
cargo install --path .
```

## 使い方

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
