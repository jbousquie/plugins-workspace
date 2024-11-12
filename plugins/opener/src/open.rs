// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Types and functions related to shell.

use serde::{Deserialize, Deserializer};

use std::{ffi::OsStr, fmt::Display, path::Path, str::FromStr};

/// Program to use on the [`open()`] call.
#[derive(Debug)]
pub enum Program {
    /// Use the `open` program.
    Open,
    /// Use the `start` program.
    Start,
    /// Use the `xdg-open` program.
    XdgOpen,
    /// Use the `gio` program.
    Gio,
    /// Use the `gnome-open` program.
    GnomeOpen,
    /// Use the `kde-open` program.
    KdeOpen,
    /// Use the `wslview` program.
    WslView,
    /// Use the `Firefox` program.
    Firefox,
    /// Use the `Google Chrome` program.
    Chrome,
    /// Use the `Chromium` program.
    Chromium,
    /// Use the `Safari` program.
    Safari,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Open => "open",
                Self::Start => "start",
                Self::XdgOpen => "xdg-open",
                Self::Gio => "gio",
                Self::GnomeOpen => "gnome-open",
                Self::KdeOpen => "kde-open",
                Self::WslView => "wslview",
                Self::Firefox => "firefox",
                Self::Chrome => "chrome",
                Self::Chromium => "chromium",
                Self::Safari => "safari",
            }
        )
    }
}

impl FromStr for Program {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = match s.to_lowercase().as_str() {
            "open" => Self::Open,
            "start" => Self::Start,
            "xdg-open" => Self::XdgOpen,
            "gio" => Self::Gio,
            "gnome-open" => Self::GnomeOpen,
            "kde-open" => Self::KdeOpen,
            "wslview" => Self::WslView,
            "firefox" => Self::Firefox,
            "chrome" | "google chrome" => Self::Chrome,
            "chromium" => Self::Chromium,
            "safari" => Self::Safari,
            _ => return Err(crate::Error::UnknownProgramName(s.to_string())),
        };
        Ok(p)
    }
}

impl<'de> Deserialize<'de> for Program {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Program::from_str(&s).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

impl Program {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Start => "start",
            Self::XdgOpen => "xdg-open",
            Self::Gio => "gio",
            Self::GnomeOpen => "gnome-open",
            Self::KdeOpen => "kde-open",
            Self::WslView => "wslview",

            #[cfg(target_os = "macos")]
            Self::Firefox => "Firefox",
            #[cfg(not(target_os = "macos"))]
            Self::Firefox => "firefox",

            #[cfg(target_os = "macos")]
            Self::Chrome => "Google Chrome",
            #[cfg(not(target_os = "macos"))]
            Self::Chrome => "google-chrome",

            #[cfg(target_os = "macos")]
            Self::Chromium => "Chromium",
            #[cfg(not(target_os = "macos"))]
            Self::Chromium => "chromium",

            #[cfg(target_os = "macos")]
            Self::Safari => "Safari",
            #[cfg(not(target_os = "macos"))]
            Self::Safari => "safari",
        }
    }
}

pub(crate) fn open<P: AsRef<OsStr>>(path: P, with: Option<Program>) -> crate::Result<()> {
    match with.map(Program::name) {
        Some(program) => ::open::with_detached(path, program),
        None => ::open::that_detached(path),
    }
    .map_err(Into::into)
}

/// Opens URL with the program specified in `with`, or system default if `None`.
///
/// # Examples
///
/// ```rust,no_run
/// tauri::Builder::default()
///   .setup(|app| {
///     // open the given URL on the system default browser
///     tauri_plugin_opener::open_url("https://github.com/tauri-apps/tauri", None)?;
///     Ok(())
///   });
/// ```
pub fn open_url<P: AsRef<str>>(url: P, with: Option<Program>) -> crate::Result<()> {
    let url = url.as_ref();
    open(url, with)
}

/// Opens path with the program specified in `with`, or system default if `None`.
///
/// # Examples
///
/// ```rust,no_run
/// tauri::Builder::default()
///   .setup(|app| {
///     // open the given URL on the system default browser
///     tauri_plugin_opener::open_path("/path/to/file", None)?;
///     Ok(())
///   });
/// ```
pub fn open_path<P: AsRef<Path>>(path: P, with: Option<Program>) -> crate::Result<()> {
    let path = path.as_ref();
    open(path, with)
}
