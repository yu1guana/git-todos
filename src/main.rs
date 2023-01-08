// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

mod error;
mod interfaces;
mod todo;

use anyhow::Result;
use interfaces::cli::Cli;

fn main() -> Result<()> {
    Cli::run()
}
