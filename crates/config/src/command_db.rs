// Copyright (C) 2024 Melody Madeline Lyons
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
//
//     Additional permission under GNU GPL version 3 section 7
//
// If you modify this Program, or any covered work, by linking or combining
// it with Steamworks API by Valve Corporation, containing parts covered by
// terms of the Steamworks API by Valve Corporation, the licensors of this
// Program grant you additional permission to convey the resulting work.

use luminol_data::commands::Command;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RMVer;

type CommandSet = HashMap<u16, Command>;
static XP_DEFAULT: Lazy<CommandSet> = Lazy::new(|| {
    let dir = luminol_macros::include_asset_dir_ids!("assets/commands/XP");
    dir.into_iter()
        .map(|(id, data)| {
            let str = std::str::from_utf8(data).unwrap();
            let cmd = ron::from_str(str).unwrap();
            (id, cmd)
        })
        .collect()
});

static VX_DEFAULT: Lazy<CommandSet> = Lazy::new(|| todo!());

static ACE_DEFAULT: Lazy<CommandSet> = Lazy::new(|| todo!());

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CommandDB {
    /// Default commands
    default: CommandSet,
    /// User defined commands
    // FIXME: visible to user?
    pub user: CommandSet,
}

impl CommandDB {
    pub fn new(ver: RMVer) -> Self {
        Self {
            default: match ver {
                RMVer::XP => &*XP_DEFAULT,
                RMVer::VX => &*VX_DEFAULT,
                RMVer::Ace => &*ACE_DEFAULT,
            }
            .clone(),
            user: HashMap::new(),
        }
    }

    pub fn get(&self, id: u16) -> Option<&Command> {
        self.user.get(&id).or_else(|| self.default.get(&id))
    }

    pub fn get_mut(&mut self, id: u16) -> Option<&mut Command> {
        self.user.get_mut(&id).or_else(|| self.default.get_mut(&id))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&u16, &Command)> {
        self.default.iter().chain(self.user.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&u16, &mut Command)> {
        self.default.iter_mut().chain(self.user.iter_mut())
    }

    pub fn len(&self) -> usize {
        self.default.len() + self.user.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
