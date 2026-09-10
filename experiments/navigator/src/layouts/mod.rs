use egui::{Rect, UiBuilder};

use crate::settings::style_sheet::LAYOUT_BLOCK_FRAME;

pub mod navigator;
pub mod title;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Layout {
    #[default]
    Title,
    Navigator,
}

type UiFn = fn(&mut egui::Ui);

#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub struct LayoutIndex(pub usize);
#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub struct ExperimentIndex(pub usize);

impl ExperimentIndex {
    pub const ALL: &'static [ExperimentIndex] = &[];
    pub const L_INDEX_TO_EXPERIMENT_INDEX: &[Option<ExperimentIndex>] = &[None, None];

    pub fn name(&self) -> &str {
        match self {
            _ => "",
        }
    }

    pub fn try_from_name(name: &str) -> Result<Self, &'static str> {
        match name {
            _ => Err("No experiment of name: {name}"),
        }
    }
}

impl Layout {
    pub const ALL: &'static [Layout] = &[Self::Title, Self::Navigator];

    pub fn name(&self) -> &str {
        match self {
            Self::Title => "title",
            Self::Navigator => "navigator",
        }
    }

    pub fn try_from_name(name: &str) -> Result<Self, &'static str> {
        match name {
            "title" => Ok(Self::Title),
            "navigator" => Ok(Self::Navigator),
            _ => Err("No layout of name: {name}"),
        }
    }

    pub fn panels(&self, max_rect: Rect) -> Vec<(Rect, UiFn)> {
        match self {
            Self::Title => title::panels(max_rect),
            Self::Navigator => navigator::panels(max_rect),
        }
    }

    pub fn ui(
        ui: &mut egui::Ui,
        from_layout: &Option<Layout>,
        to_layout: &Option<Layout>,
        t_delta: f32,
    ) {
        let max_rect: Rect = ui.max_rect();

        let start_panels: Vec<(Rect, UiFn)> = from_layout
            .as_ref()
            .map(|l| l.panels(max_rect))
            .unwrap_or_default();

        let tarpanels: Vec<(Rect, UiFn)> = to_layout
            .as_ref()
            .map(|l| l.panels(max_rect))
            .unwrap_or_default();

        let eased_t: f32 = egui::emath::easing::quadratic_in_out(if from_layout == to_layout {
            1.0
        } else {
            t_delta
        });

        let max_len: usize = start_panels.len().max(tarpanels.len());

        for i in 0..max_len {
            let start_panel: Option<&(Rect, UiFn)> = start_panels.get(i);
            let target_panel: Option<&(Rect, UiFn)> = tarpanels.get(i);

            let start_rect: Rect = start_panel.map(|(r, _)| *r).unwrap_or_else(|| {
                target_panel
                    .map(|(r, _)| Rect::from_center_size(r.center(), egui::Vec2::ZERO))
                    .unwrap_or(Rect::ZERO)
            });

            let target_rect: Rect = target_panel.map(|(r, _)| *r).unwrap_or_else(|| {
                start_panel
                    .map(|(r, _)| Rect::from_center_size(r.center(), egui::Vec2::ZERO))
                    .unwrap_or(Rect::ZERO)
            });

            let current_rect: Rect = Rect::from_min_max(
                start_rect.min.lerp(target_rect.min, eased_t),
                start_rect.max.lerp(target_rect.max, eased_t),
            );

            let active_fn: Option<UiFn> = if eased_t > 0.5 {
                target_panel
                    .map(|(_, f)| *f)
                    .or(start_panel.map(|(_, f)| *f))
            } else {
                start_panel
                    .map(|(_, f)| *f)
                    .or(target_panel.map(|(_, f)| *f))
            };

            if let Some(draw_fn) = active_fn {
                let mut box_ui = ui.new_child(UiBuilder::new().max_rect(current_rect));
                LAYOUT_BLOCK_FRAME.show(&mut box_ui, |ui| {
                    draw_fn(ui);
                });
            }
        }
    }
}

pub fn to_rect(a: (f32, f32, f32, f32), max_rect: Rect) -> Rect {
    Rect::from_min_max(
        egui::pos2(
            max_rect.min.x + a.0 * max_rect.width(),
            max_rect.min.y + a.1 * max_rect.height(),
        ),
        egui::pos2(
            max_rect.min.x + a.2 * max_rect.width(),
            max_rect.min.y + a.3 * max_rect.height(),
        ),
    )
}
