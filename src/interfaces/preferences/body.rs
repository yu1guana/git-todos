// Copyright (c) 2022 Yuichi Ishida <yu1guana@gmail.com>

use super::Language;
use getset::CopyGetters;

#[derive(Default, CopyGetters)]
pub(crate) struct Preferences {
    #[get_copy = "pub(crate)"]
    language: Language,
}
