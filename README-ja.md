# todos

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
git-todos 0.1.0

A Git based Todos App

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
