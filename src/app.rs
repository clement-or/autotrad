// The app represents all the UI logic
pub mod states;

pub struct App {
    cur_state: Box<dyn eframe::App>,
}

impl App {}

impl Default for App {
    fn default() -> Self {
        Self {
            cur_state: Box::new(states::SelectRegion {}),
        }
    }
}

#[allow(unused_must_use)]
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.cur_state.update(ctx, _frame);
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0., 0., 0., 0.5]
    }
}
