// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use crate::interfaces::preferences::Language;
use crate::interfaces::preferences::Preferences;

macro_rules! prompt {
    () => {
        ">> "
    };
}

macro_rules! separator {
    () => {
        ": "
    };
}

pub(crate) const SELECT_POINTER: &str = "*";

pub(crate) fn whitespace_with_width(width: usize) -> String {
    format!("{: ^1$}", "", width)
}

pub(crate) fn title_entry_head(pointer: &str, idx: Option<usize>) -> String {
    match idx {
        Some(idx) => format!(" {} {:>2}) ", pointer, idx),
        None => format!(" {} {:>2}  ", pointer, ""),
    }
}

pub(crate) fn no_todo_list(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => "No To-Do list",
        Language::Japanese => "TODOリストがありません",
    }
}

pub(crate) fn todo_entry_is_added(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => "A todo is added",
        Language::Japanese => "TODOが追加されました",
    }
}

pub(crate) fn todo_entry_is_removed(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => "The selected todo is removed",
        Language::Japanese => "選択されたTODOが削除されました",
    }
}

pub(crate) fn todo_entry_is_edited(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => "The selected todo is edited",
        Language::Japanese => "選択されたTODOが編集されました",
    }
}

pub(crate) fn commit_is_done(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => "Commit is done",
        Language::Japanese => "コミットされました",
    }
}

pub(crate) fn please_select_todo(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => "Select a To-Do (use Up and Down keys)",
        Language::Japanese => "上下の矢印キーを利用してTODOを選んでください",
    }
}

pub(crate) fn todo_list_is_empty(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => "a To-Do list is empty",
        Language::Japanese => "TODOリストが空です",
    }
}

pub(crate) fn title_in_add_command(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => concat!("Title (Required) ", prompt!()),
        Language::Japanese => concat!("タイトル（必須）", prompt!()),
    }
}

pub(crate) fn description_in_add_command(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => concat!("Description ", prompt!()),
        Language::Japanese => concat!("説明 ", prompt!()),
    }
}

pub(crate) fn change_type_in_finish_command(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => concat!("Type (Required)", prompt!()),
        Language::Japanese => concat!("Type（必須）", prompt!()),
    }
}

pub(crate) fn scope_in_finish_command(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => concat!("Scope ", prompt!()),
        Language::Japanese => concat!("Scope ", prompt!()),
    }
}

pub(crate) fn short_description_in_finish_command(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => concat!(
            "Short description (default is the title of the selected To-Do) ",
            prompt!()
        ),
        Language::Japanese => concat!(
            "Short description (デフォルトは選択されたTODOのタイトル) ",
            prompt!()
        ),
    }
}

pub(crate) fn long_description_in_finish_command(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => concat!(
            "Long description (default is the description of the selected To-Do) ",
            prompt!()
        ),
        Language::Japanese => concat!(
            "Long description (デフォルトは選択されたTODOの説明) ",
            prompt!()
        ),
    }
}

pub(crate) fn selected_title(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => concat!("Selected To-Do", separator!()),
        Language::Japanese => concat!("選択されたTODO", separator!()),
    }
}

pub(crate) fn last_update(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => concat!("last updated datetime", separator!()),
        Language::Japanese => concat!("最終更新時刻", separator!()),
    }
}

pub(crate) fn description(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => concat!("description", separator!()),
        Language::Japanese => concat!("説明", separator!()),
    }
}

pub(crate) fn comfirmation_prompt(preferences: &Preferences) -> &'static str {
    match preferences.language() {
        Language::English => concat!("Are you sure [y/n]? ", prompt!()),
        Language::Japanese => concat!("実行しますか [y/n]？ ", prompt!()),
    }
}
