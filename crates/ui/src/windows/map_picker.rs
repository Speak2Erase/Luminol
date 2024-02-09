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
//
//     Additional permission under GNU GPL version 3 section 7
//
// If you modify this Program, or any covered work, by linking or combining
// it with Steamworks API by Valve Corporation, containing parts covered by
// terms of the Steamworks API by Valve Corporation, the licensors of this
// Program grant you additional permission to convey the resulting work.

/// The map picker window.
/// Displays a list of maps in a tree.
/// Maps can be double clicked to open them in a map editor.
#[derive(Default)]
pub struct Window {}

// FIXME use a better iterator instead
impl Window {
    fn render_submap(
        id: indextree::NodeId,
        mapinfos: &mut luminol_data::rpg::MapInfos,
        open_map_id: &mut Option<usize>,
        ui: &mut egui::Ui,
    ) {
        let has_children = id.children(&mapinfos.arena).next().is_some();
        // Does this map have children?
        if has_children {
            // We get the map name. It's assumed that there is in fact a map with this ID in mapinfos.
            let map_info_node = mapinfos.arena.get_mut(id).unwrap();
            let map_info = map_info_node.get_mut();

            // Render a custom collapsing header.
            // It's custom so we can add a button to open a map.
            let header = egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                ui.make_persistent_id(egui::Id::new("luminol_map_info").with(id)),
                map_info.expanded,
            );

            map_info.expanded = header.openness(ui.ctx()) >= 1.;

            header
                .show_header(ui, |ui| {
                    // Has the user
                    if ui.text_edit_singleline(&mut map_info.name).double_clicked() {
                        *open_map_id = Some(map_info.map_id)
                    }
                })
                .body(|ui| {
                    for id in id.children(&mapinfos.arena).collect::<Vec<_>>() {
                        // Render children.
                        Self::render_submap(id, mapinfos, open_map_id, ui);
                    }
                });
        } else {
            // Just display a label otherwise.
            ui.horizontal(|ui| {
                let map_info_node = mapinfos.arena.get_mut(id).unwrap();
                let map_info = map_info_node.get_mut();

                ui.add_space(ui.spacing().indent);
                if ui.text_edit_singleline(&mut map_info.name).double_clicked() {
                    *open_map_id = Some(map_info.map_id)
                }
            });
        }
    }
}

impl luminol_core::Window for Window {
    fn id(&self) -> egui::Id {
        egui::Id::new("Map Picker")
    }

    fn show(
        &mut self,
        ctx: &egui::Context,
        open: &mut bool,
        update_state: &mut luminol_core::UpdateState<'_>,
    ) {
        let mut window_open = true;
        egui::Window::new("Map Picker")
            .open(&mut window_open)
            .show(ctx, |ui| {
                egui::ScrollArea::both()
                    .id_source(
                        update_state
                            .project_config
                            .as_ref()
                            .expect("project not loaded")
                            .project
                            .persistence_id,
                    )
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        // Aquire the data cache.
                        let mut mapinfos = update_state.data.map_infos();

                        let mut open_map_id = None;

                        // Now we can actually render all maps.
                        egui::CollapsingHeader::new("root")
                            .default_open(true)
                            .show(ui, |ui| {
                                // borrow checker stuff
                                let iter = mapinfos
                                    .root_id
                                    .children(&mapinfos.arena)
                                    .collect::<Vec<_>>();
                                for id in iter {
                                    Self::render_submap(id, &mut mapinfos, &mut open_map_id, ui);
                                }
                            });

                        drop(mapinfos);

                        if let Some(id) = open_map_id {
                            match crate::tabs::map::Tab::new(id, update_state) {
                                Ok(tab) => update_state.edit_tabs.add_tab(tab),
                                Err(e) => luminol_core::error!(
                                    update_state.toasts,
                                    e.wrap_err("Error enumerating maps")
                                ),
                            }
                        }
                    })
            });
        *open = window_open;
    }

    fn requires_filesystem(&self) -> bool {
        true
    }
}
