//! This module provides platform related functions.

#[cfg(unix)]
#[cfg(feature = "events")]
pub use self::unix::position;
#[cfg(windows)]
pub(crate) use self::windows::move_down;
#[cfg(windows)]
pub(crate) use self::windows::move_left;
#[cfg(windows)]
pub(crate) use self::windows::move_right;
#[cfg(windows)]
pub(crate) use self::windows::move_to;
#[cfg(windows)]
pub(crate) use self::windows::move_to_column;
#[cfg(windows)]
pub(crate) use self::windows::move_to_next_line;
#[cfg(windows)]
pub(crate) use self::windows::move_to_previous_line;
#[cfg(windows)]
pub(crate) use self::windows::move_to_row;
#[cfg(windows)]
pub(crate) use self::windows::move_up;
#[cfg(windows)]
#[cfg(feature = "events")]
pub use self::windows::position;
#[cfg(windows)]
pub(crate) use self::windows::restore_position;
#[cfg(windows)]
pub(crate) use self::windows::save_position;
#[cfg(windows)]
pub(crate) use self::windows::show_cursor;

#[cfg(windows)]
pub(crate) mod windows;

#[cfg(unix)]
#[cfg(feature = "events")]
pub(crate) mod unix;
