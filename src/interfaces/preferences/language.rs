// Copyright (c) 2022 Yuichi Ishida <yu1guana@gmail.com>

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
