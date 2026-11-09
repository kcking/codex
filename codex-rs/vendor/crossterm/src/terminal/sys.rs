//! This module provides platform related functions.

#[cfg(unix)]
pub(crate) use self::unix::disable_raw_mode;
#[cfg(unix)]
pub(crate) use self::unix::enable_raw_mode;
#[cfg(unix)]
pub(crate) use self::unix::is_raw_mode_enabled;
#[cfg(unix)]
pub(crate) use self::unix::size;
#[cfg(unix)]
#[cfg(feature = "events")]
pub use self::unix::supports_keyboard_enhancement;
#[cfg(unix)]
pub(crate) use self::unix::window_size;
#[cfg(windows)]
pub(crate) use self::windows::clear;
#[cfg(windows)]
pub(crate) use self::windows::disable_raw_mode;
#[cfg(windows)]
pub(crate) use self::windows::enable_raw_mode;
#[cfg(windows)]
pub(crate) use self::windows::is_raw_mode_enabled;
#[cfg(windows)]
pub(crate) use self::windows::scroll_down;
#[cfg(windows)]
pub(crate) use self::windows::scroll_up;
#[cfg(windows)]
pub(crate) use self::windows::set_size;
#[cfg(windows)]
pub(crate) use self::windows::set_window_title;
#[cfg(windows)]
pub(crate) use self::windows::size;
#[cfg(windows)]
#[cfg(feature = "events")]
pub use self::windows::supports_keyboard_enhancement;
#[cfg(all(windows, test))]
pub(crate) use self::windows::temp_screen_buffer;
#[cfg(windows)]
pub(crate) use self::windows::window_size;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub mod file_descriptor;
#[cfg(unix)]
mod unix;
