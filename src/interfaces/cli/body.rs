// Copyright (c) 2022 Yuichi Ishida <yu1guana@gmail.com>

use super::functions;
use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = ""
)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    #[clap(about = "Add a new Todo")]
    Add,
    #[clap(about = "Fnish a Todo and commit stated changes")]
    Finish,
    #[clap(about = "List available Todos")]
    List,
    #[clap(about = "Import an issue from remote Provider (ie. GitHub) as Todo")]
    Import,
    #[clap(about = "Mark a single Todo")]
    Mark,
    #[clap(about = "Remove existing Todo")]
    Remove,
    #[clap(about = "Show Todo details")]
    Show,
}

impl Cli {
    pub(crate) fn run() -> Result<()> {
        let preferences = Default::default();
        let todos_file_path = Self::search_todos_file_path()?;
        match Cli::parse().action {
            Action::List => functions::list(preferences, todos_file_path),
            Action::Add => functions::add(preferences, todos_file_path),
            Action::Finish => functions::finish(preferences, todos_file_path),
            Action::Remove => functions::remove(preferences, todos_file_path),
            Action::Show => functions::show(preferences, todos_file_path),
            Action::Import => {
                unimplemented!();
                Ok(())
            }
            Action::Mark => {
                unimplemented!();
                Ok(())
            }
        }
    }
    fn search_todos_file_path() -> Result<PathBuf> {
        const TODOS_FILE_NAME: &str = ".todos.toml";
        const GIT_DIR_NAME: &str = ".git";
        let mut dir = env::current_dir()?.to_path_buf();
        if dir.join(GIT_DIR_NAME).is_dir() {
            return Ok(dir.join(TODOS_FILE_NAME));
        }
        while let Some(parent_dir) = dir.parent() {
            if parent_dir.join(GIT_DIR_NAME).is_dir() {
                return Ok(parent_dir.join(TODOS_FILE_NAME));
            }
            dir = parent_dir.to_path_buf();
        }
        Err(anyhow!(
            "fatal: not a git repository (or any of the parent directories)"
        ))
    }
}
