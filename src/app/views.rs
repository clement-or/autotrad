use egui::epaint::RectShape;
use egui::Ui;
use egui::{emath, CentralPanel};
use egui::{Color32, Pos2, Rect, Rounding, Sense, Stroke};

#[derive(PartialEq, Debug)]
pub enum Event {
    Nothing,
    SelectRegionFinished { region_selected: egui::Rect },
}

pub trait AppView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> Event;
}

//
// --- Default state
//
#[derive(Default)]
pub struct Default {}

//
// --- Select region state
//

pub struct SelectRegion {
    selection: egui::Rect,
}

impl std::default::Default for SelectRegion {
    fn default() -> Self {
        Self {
            selection: egui::Rect::NOTHING,
        }
    }
}

impl SelectRegion {
    fn on_panel_shown(&mut self, ui: &mut Ui) -> Event {
        let (res, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, res.rect.size()),
            res.rect,
        );

        if let Some(rect) = {
            let is_pressing_leftclick =
                ui.input(|i| i.pointer.button_down(egui::PointerButton::Primary));

            if res.dragged() && is_pressing_leftclick {
                let (start_pos, cur_pos) = ui.input(|i| {
                    (
                        i.pointer.press_origin().unwrap_or_default(),
                        i.pointer.interact_pos().unwrap_or_default(),
                    )
                });

                Some(Rect {
                    min: Pos2::min(start_pos, cur_pos),
                    max: Pos2::max(start_pos, cur_pos),
                })
            } else {
                None
            }
        } {
            self.selection = rect
        }

        let shape = RectShape {
            rect: self.selection,
            rounding: Rounding::default(),
            fill: Color32::TRANSPARENT,
            stroke: Stroke {
                width: 1.0,
                color: Color32::RED,
            },
        };

        painter.add(shape);

        if res.drag_released_by(egui::PointerButton::Primary) {
            println!("{:?}", self.selection);
            return Event::SelectRegionFinished {
                region_selected: self.selection,
            };
        }
        Event::Nothing
    }
}

impl AppView for SelectRegion {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> Event {
        let frame = egui::Frame::default();
        {
            let mut this = frame;
            let fill = Color32::TRANSPARENT;
            this.fill = fill;
            this
        };

        let mut panel = CentralPanel::default();
        panel = panel.frame(frame);

        panel.show(ctx, |ui| {
            return self.on_panel_shown(ui);
        });

        Event::Nothing
    }
}
