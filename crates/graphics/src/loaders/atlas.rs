// Copyright (C) 2023 Lily Lyons
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
use crate::{primitives::tiles::AUTOTILE_AMOUNT, Atlas, GraphicsState, Texture};
use std::sync::{Arc, Weak};

#[derive(Default)]
pub struct Loader {
    atlases: dashmap::DashMap<usize, WeakAtlas>,
}

struct WeakAtlas {
    atlas_texture: Weak<Texture>,
    autotile_width: u32,
    tileset_height: u32,
    autotile_frames: [u32; AUTOTILE_AMOUNT as usize],
}

impl WeakAtlas {
    fn upgrade(&self) -> Option<Atlas> {
        self.atlas_texture.upgrade().map(|atlas_texture| Atlas {
            atlas_texture,
            autotile_width: self.autotile_width,
            tileset_height: self.tileset_height,
            autotile_frames: self.autotile_frames,
        })
    }

    fn from_atlas(atlas: &Atlas) -> Self {
        WeakAtlas {
            atlas_texture: Arc::downgrade(&atlas.atlas_texture),
            autotile_width: atlas.autotile_width,
            tileset_height: atlas.tileset_height,
            autotile_frames: atlas.autotile_frames,
        }
    }
}

impl Loader {
    pub fn load_atlas(
        &self,
        graphics_state: &GraphicsState,
        filesystem: &impl luminol_filesystem::FileSystem,
        tileset: &luminol_data::rpg::Tileset,
    ) -> color_eyre::Result<Atlas> {
        self.atlases
            .get(&tileset.id)
            .as_deref()
            .and_then(WeakAtlas::upgrade)
            .map(Ok)
            .unwrap_or_else(|| {
                let atlas = Atlas::new(graphics_state, filesystem, tileset);
                let weak_atlas = WeakAtlas::from_atlas(&atlas);
                self.atlases.insert(tileset.id, weak_atlas);
                Ok(atlas)
            })
    }

    pub fn reload_atlas(
        &self,
        graphics_state: &GraphicsState,
        filesystem: &impl luminol_filesystem::FileSystem,
        tileset: &luminol_data::rpg::Tileset,
    ) -> color_eyre::Result<Atlas> {
        let atlas = Atlas::new(graphics_state, filesystem, tileset);
        let weak_atlas = WeakAtlas::from_atlas(&atlas);
        self.atlases.insert(tileset.id, weak_atlas);
        Ok(atlas)
    }

    pub fn get_atlas(&self, id: usize) -> Option<Atlas> {
        self.atlases
            .get(&id)
            .as_deref()
            .and_then(WeakAtlas::upgrade)
    }

    pub fn get_expect(&self, id: usize) -> Atlas {
        self.atlases
            .get(&id)
            .as_deref()
            .and_then(WeakAtlas::upgrade)
            .expect("Atlas not loaded!")
    }

    pub fn clear(&self) {
        self.atlases.clear()
    }
}
