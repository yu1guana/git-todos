// Copyright (c) 2022 Yuichi Ishida <yu1guana@gmail.com>

#[derive(thiserror::Error, Debug)]
pub(crate) enum InterfaceError {
    #[error("Keyboard interrupted")]
    KeyboardInterrupted,
    #[error("Terminal window size must not be changed")]
    TerminalResizing,
    #[error("Terminal window is too small")]
    TooSmallTerminal,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum GitError {
    #[error("fatal: not a git repository (or any of the parent directories)")]
    NotGitRepository,
    #[error("failed to git add")]
    GitAdd,
    #[error("failed to git commit")]
    GitCommit,
}
