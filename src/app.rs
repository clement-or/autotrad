// The app represents all the UI logic
mod views;

use eframe::egui;

pub struct App {
    cur_view: Box<dyn views::AppView>,
    cur_state: State,
}

impl App {}

impl Default for App {
    fn default() -> Self {
        Self {
            cur_view: Box::new(views::SelectRegion::default()),
            cur_state: State::None,
        }
    }
}

#[allow(unused_must_use)]
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.cur_state {
            State::Default => self.cur_view = Box::new(views::SelectRegion::default()),
            State::PendingRegionSelection => (),
            _ => (),
        }
        let event = self.cur_view.update(ctx, _frame);

        self.cur_state.run();
        let new_state = self.cur_state.next(&event);

        if self.cur_state != new_state {
            self.cur_state.exit();
            new_state.enter();
            self.cur_state = new_state;
        }
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0., 0., 0., 0.5]
    }
}

#[derive(PartialEq, Debug)]
enum State {
    None,
    Default,
    PendingRegionSelection,
}

impl State {
    fn next(&self, event: &views::Event) -> State {
        match (self, event) {
            _ => State::Default,
        }
    }

    fn run(&self) {}

    fn exit(&self) {}

    fn enter(&self) {}
}
