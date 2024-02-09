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

use serde::ser::SerializeMap;

#[derive(Debug, PartialEq, Eq)]
pub struct MapInfos {
    pub arena: indextree::Arena<MapInfo>,
    pub root_id: indextree::NodeId,
    pub modified: bool,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct MapInfo {
    pub name: String,
    pub map_id: usize,
    pub expanded: bool,
    pub scroll_x: i32,
    pub scroll_y: i32,
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(rename = "RPG::MapInfo")]
pub struct RawMapInfo {
    pub name: String,
    // because mapinfos is stored in a hash, we dont actually need to modify values! this can just stay as a usize.
    // it would be slightly more accurate to store this as an option, but no other values (off the top of my head) are like this. maybe event tile ids.
    // I'll need to think on this a bit.
    pub parent_id: usize,
    pub order: i32,
    pub expanded: bool,
    pub scroll_x: i32,
    pub scroll_y: i32,
}

pub type RawMapInfos = std::collections::HashMap<usize, RawMapInfo>;

impl PartialOrd for RawMapInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RawMapInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl Default for MapInfos {
    fn default() -> Self {
        let mut arena = indextree::Arena::with_capacity(16);
        let root_id = arena.new_node(Default::default());

        root_id.append_value(Default::default(), &mut arena);

        Self {
            arena,
            root_id,
            modified: false,
        }
    }
}

impl<'de> serde::Deserialize<'de> for MapInfos {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw_mapinfos = RawMapInfos::deserialize(deserializer)?;

        let mut sorted_mapinfos = raw_mapinfos.into_iter().collect::<Vec<_>>();
        // There shouldn't be any duplicates so the sort_unstable is ok here
        sorted_mapinfos.sort_unstable_by(|(_, m1), (_, m2)| m1.cmp(m2));

        let mut arena = indextree::Arena::with_capacity(sorted_mapinfos.len() + 1);
        let root_id = arena.new_node(Default::default()); // bit wasteful but oh well

        // this conversion is a tad annoying but i'm not sure there's another way?
        // it'd be great if we could go from usize to node id but we can't :/
        let mut map_ids_to_node_ids = std::collections::HashMap::new();
        map_ids_to_node_ids.insert(0, root_id);

        for (map_id, mapinfo) in sorted_mapinfos {
            let parent_id = map_ids_to_node_ids[&mapinfo.parent_id];

            let mapinfo = MapInfo {
                name: mapinfo.name,
                map_id,
                expanded: mapinfo.expanded,
                scroll_x: mapinfo.scroll_x,
                scroll_y: mapinfo.scroll_y,
            };
            let map_node_id = parent_id.append_value(mapinfo, &mut arena);
            map_ids_to_node_ids.insert(map_id, map_node_id);
        }

        Ok(Self {
            arena,
            root_id,
            modified: false,
        })
    }
}

impl serde::Serialize for MapInfos {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        fn recurse_serialize<S>(
            id: indextree::NodeId,
            parent_id: usize,
            serialize_map: &mut S,
            arena: &indextree::Arena<MapInfo>,
            order: &mut i32,
        ) -> Result<(), S::Error>
        where
            S: serde::ser::SerializeMap,
        {
            let node = &arena[id];

            let map_info = node.get();

            let raw_map_info = RawMapInfo {
                // the extra clone is annoying but it probably doesn't matter
                name: map_info.name.clone(),
                parent_id,
                order: *order,
                expanded: map_info.expanded,
                scroll_x: map_info.scroll_x,
                scroll_y: map_info.scroll_y,
            };
            serialize_map.serialize_key(&map_info.map_id)?;
            serialize_map.serialize_value(&raw_map_info)?;
            *order += 1;

            for child in id.children(arena) {
                recurse_serialize(child, map_info.map_id, serialize_map, arena, order)?;
            }

            Ok(())
        }

        let count = self.arena.count() - 1;
        let mut serialize_map = serializer.serialize_map(Some(count))?;
        let mut order = 0;

        for child in self.root_id.children(&self.arena) {
            recurse_serialize(child, 0, &mut serialize_map, &self.arena, &mut order)?;
        }

        serialize_map.end()
    }
}
