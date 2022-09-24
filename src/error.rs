// Copyright (c) 2022 Yuichi Ishida <yu1guana@gmail.com>

#[derive(thiserror::Error, Debug)]
pub enum InterfaceError {
    #[error("Keyboard interrupted")]
    KeyboardInterrupted,
    #[error("Terminal window size must not be changed")]
    TerminalResizing,
    #[error("Terminal window is too small")]
    TooSmallTerminal,
    #[error("fatal: not a git repository (or any of the parent directories)")]
    NotGitRepository,
}
