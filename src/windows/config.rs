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

use strum::IntoEnumIterator;

use crate::data::config::RGSSVer;

use super::window::Window;

/// The confg window
pub struct ConfigWindow {}

impl ConfigWindow {}

impl Window for ConfigWindow {
    fn name(&self) -> String {
        "Local Luminol Config".to_string()
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool, info: &'static crate::UpdateInfo) {
        egui::Window::new(self.name()).open(open).show(ctx, |ui| {
            let mut config = info.data_cache.config();
            let config = config.as_mut().unwrap();

            ui.label("Project name");
            ui.text_edit_singleline(&mut config.project_name);
            ui.label("Scripts path");
            ui.text_edit_singleline(&mut config.scripts_path);
            ui.checkbox(&mut config.use_ron, "Use RON (Rusty Object Notation)");
            egui::ComboBox::from_label("RGSS Version")
                .selected_text(config.rgss_ver.to_string())
                .show_ui(ui, |ui| {
                    for ver in RGSSVer::iter() {
                        ui.selectable_value(&mut config.rgss_ver, ver, ver.to_string());
                    }
                });
        });
    }

    fn requires_filesystem(&self) -> bool {
        true
    }
}
