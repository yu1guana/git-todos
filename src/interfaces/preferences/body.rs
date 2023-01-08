// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

use super::Language;
use getset::CopyGetters;

#[derive(Default, CopyGetters)]
pub(crate) struct Preferences {
    #[get_copy = "pub(crate)"]
    language: Language,
}
