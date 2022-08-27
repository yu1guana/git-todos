// Copyright (c) 2022 Yuichi Ishida <yu1guana@gmail.com>

use super::entry::TodoEntry;
use anyhow::{Context, Result};
use getset::{Getters, MutGetters};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use toml;

#[derive(Debug, Serialize, Deserialize, Getters, MutGetters)]
#[getset(get = "pub(crate)", get_mut = "pub(crate)")]
pub(crate) struct TodoList {
    todos: Option<Vec<TodoEntry>>,
}

impl TodoList {
    pub(crate) fn new() -> Self {
        Self { todos: None }
    }
    pub(crate) fn read_file(file_path: &Path) -> Result<Self> {
        let file_contents = fs::read_to_string(file_path)
            .with_context(|| format!("failed to open {}", file_path.display()))?;
        toml::from_str(&file_contents)
            .with_context(|| format!("failed to parse {}", file_path.display()))
    }
    pub(crate) fn write_file(&self, file_path: &Path) -> Result<()> {
        let mut buf_writer = BufWriter::new(
            File::create(file_path)
                .with_context(|| format!("failed to open {}", file_path.display()))?,
        );
        write!(
            buf_writer,
            "{}",
            toml::to_string(&self)
                .with_context(|| format!("failed to change todo list into toml string"))?
        )
        .with_context(|| format!("failed to write todo list in {}", file_path.display()))?;
        Ok(())
    }
    pub(crate) fn push(&mut self, todo_entry: TodoEntry) {
        match &mut self.todos {
            Some(todos) => todos.push(todo_entry),
            None => {
                self.todos = Some(vec![todo_entry]);
            }
        }
    }
}
