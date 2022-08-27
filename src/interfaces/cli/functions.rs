// Copyright (c) 2022 Yuichi Ishida <yu1guana@gmail.com>

use crate::interfaces::messages;
use crate::interfaces::messages::SELECT_POINTER;
use crate::interfaces::preferences::Preferences;
use crate::todo::{TodoEntry, TodoList};
use anyhow::{anyhow, Result};
use chrono::Utc;
use std::io;
use std::io::{Stdout, Write};
use std::path::PathBuf;
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use unicode_width::UnicodeWidthStr;

pub(crate) fn list(preferences: Preferences, todos_file_path: PathBuf) -> Result<()> {
    if todos_file_path.is_file() {
        let todo_list = TodoList::read_file(&todos_file_path)?;
        if let Some(todos) = todo_list.todos() {
            if todos.is_empty() {
                println!("{}", messages::todo_list_is_empty(&preferences));
            } else {
                let mut stdout = io::stdout().into_raw_mode()?;
                write_title_list(&mut stdout, None, todos)?;
            }
        } else {
            println!("{}", messages::todo_list_is_empty(&preferences));
        }
    } else {
        println!("{}", messages::no_todo_list(&preferences));
    }
    Ok(())
}

pub(crate) fn add(preferences: Preferences, todos_file_path: PathBuf) -> Result<()> {
    let title = one_line_reader(true, messages::title_in_add_command(&preferences))?;
    let description = one_line_reader(false, messages::description_in_add_command(&preferences))?;
    let updated_utc_datetime_rfc3339 = Utc::now().to_rfc3339();
    let todo_entry = TodoEntry::new(title, description, updated_utc_datetime_rfc3339);
    let mut todo_list = if todos_file_path.is_file() {
        TodoList::read_file(&todos_file_path)?
    } else {
        TodoList::new()
    };
    todo_list.push(todo_entry);
    todo_list.write_file(&todos_file_path)
}

pub(crate) fn remove(preferences: Preferences, todos_file_path: PathBuf) -> Result<()> {
    if todos_file_path.is_file() {
        let mut todo_list = TodoList::read_file(&todos_file_path)?;
        if let Some(todos) = todo_list.todos_mut() {
            if todos.is_empty() {
                println!("{}", messages::todo_list_is_empty(&preferences));
            } else {
                let remove_id = select(&todos)?;
                todos.remove(remove_id);
                todo_list.write_file(&todos_file_path)?;
            }
        } else {
            println!("{}", messages::todo_list_is_empty(&preferences));
        }
    } else {
        println!("{}", messages::no_todo_list(&preferences));
    }
    Ok(())
}

pub(crate) fn finish(preferences: Preferences, todos_file_path: PathBuf) -> Result<()> {
    unimplemented!();
    Ok(())
}

pub(crate) fn show(preferences: Preferences, todos_file_path: PathBuf) -> Result<()> {
    unimplemented!();
    Ok(())
}

fn one_line_reader(necessary: bool, prompt_message: &str) -> Result<String> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode()?;
    let mut s = String::new();
    let prompt_message_width = prompt_message.width_cjk() as u16;
    write!(stdout, "{}", prompt_message)?;
    stdout.flush()?;
    for key in stdin.keys() {
        match key {
            Ok(Key::Char('\n')) => {
                if necessary && s.is_empty() {
                    continue;
                } else {
                    write!(stdout, "\n")?;
                    write!(
                        stdout,
                        "{}",
                        cursor::Left(prompt_message_width + s.as_str().width_cjk() as u16)
                    )?;
                    stdout.flush()?;
                    break;
                }
            }
            Ok(Key::Char(c)) => {
                write!(stdout, "{}", clear::CurrentLine)?;
                write!(
                    stdout,
                    "{}",
                    cursor::Left(prompt_message_width + s.as_str().width_cjk() as u16)
                )?;
                s.push(c);
                write!(stdout, "{}{}", prompt_message, s)?;
                stdout.flush()?;
            }
            Ok(Key::Backspace) => {
                write!(stdout, "{}", clear::CurrentLine)?;
                write!(
                    stdout,
                    "{}",
                    cursor::Left(prompt_message_width + s.as_str().width_cjk() as u16)
                )?;
                s.pop();
                write!(stdout, "{}{}", prompt_message, s)?;
                stdout.flush()?;
            }
            Ok(Key::Ctrl('c')) => return Err(anyhow!("Keyboard interrupted")),
            _ => {}
        }
    }
    Ok(s)
}

fn select(todos: &[TodoEntry]) -> Result<usize> {
    let terminal_size = termion::terminal_size()?;
    let mut select_id = 0;
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode()?;
    let num_row = title_list_lines(Some(select_id), todos)?.len();
    write!(stdout, "{}", cursor::Hide)?;
    write_title_list(&mut stdout, Some(select_id), todos)?;
    for key in stdin.keys() {
        if termion::terminal_size()? != terminal_size {
            write!(stdout, "{}", cursor::Left(u16::MAX))?;
            write!(stdout, "{}", cursor::Show)?;
            return Err(anyhow!("Terminal size is changed in operations"));
        }
        match key {
            Ok(Key::Char('\n')) => break,
            Ok(Key::Up) => {
                select_id = select_id.saturating_sub(1);
            }
            Ok(Key::Down) => {
                if select_id != todos.len() - 1 {
                    select_id += 1
                }
            }
            Ok(Key::Ctrl('c')) => {
                write!(stdout, "{}", cursor::Show)?;
                write!(stdout, "{}", cursor::Left(u16::MAX))?;
                stdout.flush()?;
                return Err(anyhow!("Keyboard interrupted"));
            }
            _ => {}
        }
        clear_upper_lines(&mut stdout, num_row)?;
        write_title_list(&mut stdout, Some(select_id), todos)?;
    }
    write!(stdout, "{}", cursor::Show)?;
    stdout.flush()?;
    Ok(select_id)
}

fn title_list_lines(select_id: Option<usize>, todos: &[TodoEntry]) -> Result<Vec<String>> {
    let (terminal_width, _) = termion::terminal_size()?;
    let indent = messages::whitespace_with_width(SELECT_POINTER.width_cjk());
    let mut lines = vec![];
    for (i_todo, todo) in todos.into_iter().enumerate() {
        lines.push(match select_id {
            Some(id) => {
                if i_todo == id {
                    messages::title_entry_head(SELECT_POINTER, Some(i_todo))
                } else {
                    messages::title_entry_head(&indent, Some(i_todo))
                }
            }
            None => messages::title_entry_head(&indent, Some(i_todo)),
        });
        if lines.last().unwrap().width_cjk() > (terminal_width - 5) as usize {
            return Err(anyhow!("terminal window is too small"));
        }
        for c in todo.title().chars() {
            lines.last_mut().unwrap().push(c);
            if lines.last().unwrap().width_cjk() > (terminal_width - 3) as usize {
                lines.push(messages::title_entry_head(&indent, None));
            }
        }
    }
    Ok(lines)
}

fn write_title_list(
    stdout: &mut RawTerminal<Stdout>,
    select_id: Option<usize>,
    todos: &[TodoEntry],
) -> Result<()> {
    let lines = title_list_lines(select_id, todos)?;
    for line in lines {
        writeln!(stdout, "{}", line)?;
        write!(stdout, "{}", cursor::Left(line.as_str().width_cjk() as u16))?;
    }
    Ok(())
}

fn clear_upper_lines(stdout: &mut RawTerminal<Stdout>, num_row: usize) -> Result<()> {
    for _ in 0..num_row {
        write!(stdout, "{}", cursor::Up(1))?;
        write!(stdout, "{}", clear::CurrentLine)?;
    }
    Ok(())
}
