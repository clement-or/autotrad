// The app represents all the UI logic
mod views;

use eframe::egui;

use self::views::AppView;

pub struct App {
    cur_state: State,
    default_view: views::Default,
    selectregion_view: views::SelectRegion,
}

impl App {}

impl Default for App {
    fn default() -> Self {
        Self {
            cur_state: State::None,
            default_view: views::Default::default(),
            selectregion_view: views::SelectRegion::default(),
        }
    }
}

#[allow(unused_must_use)]
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let event = match self.cur_state {
            State::Default => self.selectregion_view.update(ctx, _frame),
            State::PendingRegionSelection => views::Event::Nothing,
            _ => views::Event::Nothing,
        };

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
