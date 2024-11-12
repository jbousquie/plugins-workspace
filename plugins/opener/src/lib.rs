// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::path::Path;

use tauri::{
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime,
};

#[cfg(mobile)]
use tauri::plugin::PluginHandle;
#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "app.tauri.opener";
#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_opener);

mod commands;
mod error;
mod open;
mod reveal_item_in_dir;
mod scope;
mod scope_entry;

pub use error::Error;
type Result<T> = std::result::Result<T, Error>;

pub use open::{open_path, open_url, Program};
pub use reveal_item_in_dir::reveal_item_in_dir;

pub struct Opener<R: Runtime> {
    #[allow(dead_code)]
    app: AppHandle<R>,
    #[cfg(mobile)]
    mobile_plugin_handle: PluginHandle<R>,
}

impl<R: Runtime> Opener<R> {
    /// Open a  url with a default or specific browser opening program.
    #[cfg(desktop)]
    pub fn open_url(&self, url: impl Into<String>, with: Option<open::Program>) -> Result<()> {
        open::open(url.into(), with).map_err(Into::into)
    }

    /// Open a  url with a default or specific browser opening program.
    #[cfg(mobile)]
    pub fn open_url(&self, url: impl Into<String>, _with: Option<open::Program>) -> Result<()> {
        self.mobile_plugin_handle
            .run_mobile_plugin("open", url.into())
            .map_err(Into::into)
    }

    /// Open a (url) path with a default or specific browser opening program.
    #[cfg(desktop)]
    pub fn open_path(&self, path: impl Into<String>, with: Option<open::Program>) -> Result<()> {
        open::open(path.into(), with).map_err(Into::into)
    }

    /// Open a (url) path with a default or specific browser opening program.
    #[cfg(mobile)]
    pub fn open_path(&self, path: impl Into<String>, _with: Option<open::Program>) -> Result<()> {
        self.mobile_plugin_handle
            .run_mobile_plugin("open", path.into())
            .map_err(Into::into)
    }

    pub fn reveal_item_in_dir<P: AsRef<Path>>(&self, p: P) -> Result<()> {
        reveal_item_in_dir::reveal_item_in_dir(p)
    }
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`], [`tauri::WebviewWindow`], [`tauri::Webview`] and [`tauri::Window`] to access the opener APIs.
pub trait OpenerExt<R: Runtime> {
    fn opener(&self) -> &Opener<R>;
}

impl<R: Runtime, T: Manager<R>> crate::OpenerExt<R> for T {
    fn opener(&self) -> &Opener<R> {
        self.state::<Opener<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("opener")
        .js_init_script(include_str!("init-iife.js").to_string())
        .setup(|app, _api| {
            #[cfg(target_os = "android")]
            let handle = _api.register_android_plugin(PLUGIN_IDENTIFIER, "OpenerPlugin")?;
            #[cfg(target_os = "ios")]
            let handle = _api.register_ios_plugin(init_plugin_opener)?;

            app.manage(Opener {
                app: app.clone(),
                #[cfg(mobile)]
                mobile_plugin_handle: handle,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::open_url,
            commands::open_path,
            commands::reveal_item_in_dir
        ])
        .build()
}
