use egui::epaint::RectShape;
use egui::Ui;
use egui::{emath, CentralPanel};
use egui::{Color32, Pos2, Rect, Rounding, Sense, Stroke};

//
// --- Default state
//

pub struct Default {}

//
// --- Select region state
//

pub struct SelectRegion {}

impl SelectRegion {
    fn on_drag_finished(&mut self, region_selected: Rect) {
        println!(
            "Region selected - Start: {};{}, End: {};{}",
            region_selected.min.x,
            region_selected.min.y,
            region_selected.max.x,
            region_selected.max.y
        );
    }

    fn on_panel_shown(&mut self, ui: &mut Ui) {
        let (res, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, res.rect.size()),
            res.rect,
        );

        if res.drag_started() {}

        let is_pressing_leftclick =
            ui.input(|i| i.pointer.button_down(egui::PointerButton::Primary));

        if res.dragged() && is_pressing_leftclick {
            let (start_pos, cur_pos) = ui.input(|i| {
                (
                    i.pointer.press_origin().unwrap_or_default(),
                    i.pointer.interact_pos().unwrap_or_default(),
                )
            });

            let (min, max) = (Pos2::min(start_pos, cur_pos), Pos2::max(start_pos, cur_pos));

            let shape = RectShape {
                rect: Rect { min, max },
                rounding: Rounding::default(),
                fill: Color32::TRANSPARENT,
                stroke: Stroke {
                    width: 1.0,
                    color: Color32::RED,
                },
            };

            painter.add(shape);
            if res.drag_released_by(egui::PointerButton::Primary) {
                self.on_drag_finished(Rect { min, max });
            }
        };
    }
}

impl eframe::App for SelectRegion {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            self.on_panel_shown(ui);
        });
    }
}
