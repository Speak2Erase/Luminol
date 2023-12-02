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

#![allow(missing_docs)]

use egui::{
    text::{LayoutJob, LayoutSection},
    FontId, TextFormat,
};
use prism::Visit;

/// View some code with syntax highlighting and selection.
pub fn code_view_ui(ui: &mut egui::Ui, mut code: &str, theme: luminol_config::CodeTheme) {
    let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
        let layout_job = highlight(ui.ctx(), theme, string);
        // layout_job.wrap.max_width = wrap_width; // no wrapping
        ui.fonts(|f| f.layout_job(layout_job))
    };

    ui.add(
        egui::TextEdit::multiline(&mut code)
            .font(egui::TextStyle::Monospace) // for cursor height
            .code_editor()
            .desired_rows(1)
            .lock_focus(true)
            .layouter(&mut layouter),
    );
}

/// Memoized Code highlighting
#[must_use]
pub fn highlight(ctx: &egui::Context, theme: luminol_config::CodeTheme, code: &str) -> LayoutJob {
    impl egui::util::cache::ComputerMut<(luminol_config::CodeTheme, &str), LayoutJob> for Highlighter {
        fn compute(&mut self, (theme, code): (luminol_config::CodeTheme, &str)) -> LayoutJob {
            self.highlight(theme, code)
        }
    }

    type HighlightCache = egui::util::cache::FrameCache<LayoutJob, Highlighter>;

    ctx.memory_mut(|m| {
        let highlight_cache = m.caches.cache::<HighlightCache>();
        highlight_cache.get((theme, code))
    })
}

#[derive(Default)]
struct Highlighter {}

struct Visitor<'job> {
    job: &'job mut LayoutJob,
}

impl Highlighter {
    #[allow(clippy::unused_self, clippy::unnecessary_wraps)]
    fn highlight(&mut self, theme: luminol_config::CodeTheme, code: &str) -> LayoutJob {
        self.highlight_impl(theme, code).unwrap_or_else(|| {
            // Fallback:
            LayoutJob::simple(
                code.into(),
                egui::FontId::monospace(12.0),
                if theme.dark_mode {
                    egui::Color32::LIGHT_GRAY
                } else {
                    egui::Color32::DARK_GRAY
                },
                f32::INFINITY,
            )
        })
    }

    fn highlight_impl(
        &mut self,
        theme: luminol_config::CodeTheme,
        text: &str,
    ) -> Option<LayoutJob> {
        let mut job = LayoutJob {
            text: text.into(),
            ..Default::default()
        };

        let results = prism::parse(text.as_bytes());
        for diagnostic in results.errors() {
            eprintln!("{}", diagnostic.message());
        }
        println!("{:#?}", results.node());

        let mut visitor = Visitor { job: &mut job };
        visitor.visit(&results.node());

        Some(job)
    }
}

impl<'jobs> Visitor<'jobs> {
    fn keyword(&mut self, location: prism::Location<'_>) {
        self.job.sections.push(LayoutSection {
            leading_space: 0.0,
            byte_range: location_range(location),
            format: TextFormat {
                font_id: FontId::monospace(12.0),
                color: egui::Color32::from_rgb(204, 153, 204),
                ..Default::default()
            },
        });
    }
}

impl<'jobs, 'node> prism::Visit<'node> for Visitor<'jobs> {
    fn visit_class_node(&mut self, node: &prism::ClassNode<'node>) {
        self.keyword(node.class_keyword_loc());
        prism::visit_class_node(self, node);
        self.keyword(node.end_keyword_loc());
    }
}

fn location_range(location: prism::Location) -> std::ops::Range<usize> {
    location.start_offset()..location.end_offset()
}
