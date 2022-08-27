// Copyright (c) 2022 Yuichi Ishida <yu1guana@gmail.com>

use crate::interfaces::preferences::Language;
use crate::interfaces::preferences::Preferences;

macro_rules! prompt {
    () => {
        ">> "
    };
}

pub(super) const SELECT_POINTER: &'static str = "*";

pub(super) fn whitespace_with_width(width: usize) -> String {
    format!("{: ^1$}", "", width)
}

pub(super) fn title_entry_head(pointer: &str, idx: Option<usize>) -> String {
    match idx {
        Some(idx) => format!(" {} {:>3}) ", pointer, idx),
        None => format!(" {} {:>3}  ", pointer, ""),
    }
}

pub(super) fn no_todo_list(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => "No To-Do list",
        Language::Japanese => "TODOリストがありません",
    }
}

pub(super) fn todo_list_is_empty(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => "a To-Do list is empty",
        Language::Japanese => "TODOリストが空です",
    }
}

pub(super) fn title_in_add_command(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => concat!("Title (Required) ", prompt!()),
        Language::Japanese => concat!("タイトル（必須）", prompt!()),
    }
}

pub(super) fn description_in_add_command(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => concat!("Description ", prompt!()),
        Language::Japanese => concat!("説明 ", prompt!()),
    }
}