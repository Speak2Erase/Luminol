#![warn(clippy::all, rust_2018_idioms)]
// Copyright (C) 2022 Lily Lyons
// 
// This file is part of Luminol.
// 
// Luminol is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// Luminol is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with Luminol.  If not, see <http://www.gnu.org/licenses/>.


mod app;

mod audio;

mod windows {
    pub mod about;
    pub mod map_picker;
    pub mod sound_test;
    pub mod window;
}

mod components {
    pub mod map_toolbar;
    pub mod tilemap;
    pub mod toolbar;
    pub mod top_bar;
}

mod tabs {
    pub mod map;
    pub mod started;
    pub mod tab;
}

mod data {
    pub mod rgss_structs;
    pub mod rmxp_structs;
}

mod filesystem {
    #[cfg(not(target_arch = "wasm32"))]
    mod filesystem_native;
    #[cfg(target_arch = "wasm32")]
    mod filesystem_wasm32;
    #[cfg(not(target_arch = "wasm32"))]
    pub use filesystem_native::Filesystem;
    #[cfg(target_arch = "wasm32")]
    pub use filesystem_wasm32::Filesystem;
    pub mod data_cache;
    pub mod image_cache;
}

pub use app::App;

/// Embedded icon 256x256 in size.
pub const ICON: &[u8] = include_bytes!("../assets/icon-256.png");

use crate::filesystem::{data_cache::DataCache, Filesystem};
/// Passed to windows and widgets when updating.
pub struct UpdateInfo<'a> {
    pub filesystem: &'a Filesystem,
    pub data_cache: &'a DataCache,
    pub windows: &'a windows::window::Windows,
    pub tabs: &'a tabs::tab::Tabs,
    pub audio: &'a audio::Audio,
    pub images: &'a filesystem::image_cache::ImageCache,
}
