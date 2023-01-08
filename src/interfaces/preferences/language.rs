// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

#[derive(Copy, Clone)]
pub(crate) enum Language {
    Japanese,
    English,
}

impl Default for Language {
    fn default() -> Self {
        match sys_locale::get_locale() {
            Some(locale) => match locale.as_str() {
                "ja-JP" => Language::Japanese,
                _ => Language::English,
            },
            None => Language::English,
        }
    }
}
