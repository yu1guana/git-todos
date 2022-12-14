// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use super::functions;
use crate::error::GitError;
use crate::interfaces::common::names;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, after_help = concat!("Repository: ", env!("CARGO_PKG_REPOSITORY")))]
pub(crate) struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    #[clap(about = "List available Todos")]
    List,
    #[clap(about = "Show Todo details")]
    Show,
    #[clap(about = "Add a new Todo")]
    Add,
    #[clap(about = "Edit a Todo")]
    Edit,
    #[clap(about = "Fnish a Todo and commit stated changes")]
    Finish,
    #[clap(about = "Remove existing Todo")]
    Remove,
}

impl Cli {
    pub(crate) fn run() -> Result<()> {
        let preferences = Default::default();
        let git_repository_path = Self::serach_repository_path()?;
        let todos_file_path = git_repository_path.join(names::TODOS_FILE);
        match Cli::parse().action {
            Action::List => functions::list(preferences, todos_file_path),
            Action::Add => functions::add(preferences, todos_file_path),
            Action::Edit => functions::edit(preferences, todos_file_path),
            Action::Finish => functions::finish(preferences, todos_file_path),
            Action::Remove => functions::remove(preferences, todos_file_path),
            Action::Show => functions::show(preferences, todos_file_path),
        }
    }
    fn serach_repository_path() -> Result<PathBuf> {
        let mut dir = env::current_dir()?;
        if dir.join(names::GIT_DIRECTORY).is_dir() {
            return Ok(dir);
        }
        while let Some(parent_dir) = dir.parent() {
            if parent_dir.join(names::GIT_DIRECTORY).is_dir() {
                return Ok(parent_dir.to_path_buf());
            }
            dir = parent_dir.to_path_buf();
        }
        Err(GitError::NotGitRepository.into())
    }
}
