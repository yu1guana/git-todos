// Copyright (c) 2022 Yuichi Ishida <yu1guana@gmail.com>

use super::entry::TodoEntry;
use anyhow::{Context, Result};
use getset::{Getters, MutGetters};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Getters, MutGetters)]
#[getset(get = "pub(crate)", get_mut = "pub(crate)")]
pub(crate) struct TodoList {
    todos: Vec<TodoEntry>,
}

impl TodoList {
    pub(crate) fn new() -> Self {
        Self { todos: Vec::new() }
    }
    pub(crate) fn read_file(file_path: &Path) -> Result<Self> {
        let file_contents = fs::read_to_string(file_path)
            .with_context(|| format!("failed to read todos from {}", file_path.display()))?;
        toml::from_str(&file_contents)
            .with_context(|| format!("failed to parse {}", file_path.display()))
    }
    pub(crate) fn write_file(&self, file_path: &Path) -> Result<()> {
        let file_contents = toml::to_string(&self)
            .with_context(|| "failed to change todo list into toml string".to_string())?;
        fs::write(file_path, file_contents)
            .with_context(|| format!("failed to write todos into {}", file_path.display()))
    }
    pub(crate) fn push(&mut self, todo_entry: TodoEntry) {
        self.todos.push(todo_entry);
    }
}
