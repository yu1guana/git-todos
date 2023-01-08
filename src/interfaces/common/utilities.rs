// Copyright (c) 2023 Yuichi Ishida <yu1guana@gmail.com>
//
// Released under the MIT license.
// see https://opensource.org/licenses/mit-license.php

pub(crate) fn make_commit_message(
    change_type: &str,
    scope: &str,
    short_description: &str,
    long_description: &str,
) -> String {
    format!(
        "{}{}: {}{}",
        change_type,
        if scope.is_empty() {
            "".to_owned()
        } else {
            format!(" ({})", scope)
        },
        short_description,
        if long_description.is_empty() {
            "\n".to_owned()
        } else {
            format!("\n\n{}\n", long_description)
        }
    )
}

pub(crate) fn print_commit_message_with_box(box_title: &str, commit_message: &str) {
    println!("----- {} -----", box_title);
    print!("{}", commit_message);
    println!("---------------------------------");
}

pub(crate) fn print_commit_message_format_with_box() {
    print_commit_message_with_box(
        "Commit message format",
        &make_commit_message(
            "Change type",
            "Scope",
            "Short description",
            "Long description",
        ),
    );
}
