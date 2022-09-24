// Copyright (c) 2022 Yuichi Ishida <yu1guana@gmail.com>

mod error;
mod interfaces;
mod todo;

use anyhow::Result;
use interfaces::cli::Cli;

fn main() -> Result<()> {
    Cli::run()
}
