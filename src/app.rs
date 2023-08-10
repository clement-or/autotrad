// The app represents all the UI logic

use egui::emath;
use egui::epaint::RectShape;
use egui::Ui;
use egui::{CentralPanel, Color32, Pos2, Rect, Rounding, Sense, Stroke};

#[derive(std::default::Default)]
pub struct App {}

impl App {
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
        };
    }
}

#[allow(unused_must_use)]
impl eframe::App for App {
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
            App::on_panel_shown(self, ui);
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0., 0., 0., 0.5]
    }
}
