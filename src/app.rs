// The app represents all the UI logic
use eframe::egui;
use egui::epaint::RectShape;
use egui::Ui;
use egui::{emath, CentralPanel};
use egui::{Color32, Pos2, Rect, Rounding, Sense, Stroke};

const DATA_SELECTION_ID: &str = "region_selection";

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Event {
    Nothing,
    RegionSelectionUpdated,
    RegionSelectionFinished,
    SelectRegionButtonClicked,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum State {
    None,
    Default,
    SelectRegion,
}

//
// --- Statemachine transition logic
// This part handles the transitions between different states based on events received from them.
//
impl App {
    fn get_next_state(&self) -> State {
        match (self.cur_state, self.cur_event) {
            (State::SelectRegion, Event::RegionSelectionFinished) => State::Default,
            (State::Default, Event::SelectRegionButtonClicked) => State::SelectRegion,
            (_, Event::Nothing) => self.cur_state,

            (State::None, _) => State::Default,
            _ => self.cur_state,
        }
    }
}

// --- Main StateMachine logic
impl State {
    fn run(&self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> Event {
        match self {
            State::Default => State::run_default(ctx, _frame),
            State::SelectRegion => State::run_selectregion(ctx, _frame),
            _ => Event::Nothing,
        }
    }

    fn exit(&self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self {
            State::Default => State::enter_default(_ctx, _frame),
            State::SelectRegion => State::enter_selectregion(_ctx, _frame),
            _ => (),
        }
    }

    fn enter(&self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self {
            State::Default => State::exit_default(_ctx, _frame),
            State::SelectRegion => State::exit_selectregion(_ctx, _frame),
            _ => (),
        }
    }
}

//
// --- Default ui state
// Default window displayed on starting the app
//
impl State {
    fn run_default(ctx: &egui::Context, _frame: &mut eframe::Frame) -> Event {
        _frame.set_maximized(false);
        _frame.set_decorations(true);
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.add(egui::Label::new("Hello World!"));
                ui.label("A shorter and more convenient way to add a label.");
                if ui.button("Click me").clicked() {
                    Event::SelectRegionButtonClicked
                } else {
                    Event::Nothing
                }
            })
            .inner
    }
    fn enter_default(_ctx: &egui::Context, _frame: &mut eframe::Frame) {
        _frame.set_window_size(egui::Vec2 { x: 200.0, y: 150.0 });
        _frame.set_centered();
    }
    fn exit_default(_ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}

//
// --- Select region state
// In this state, we prompt the user to select a screen region and draw a rectangle defining its
// box selection. Player exits the state when releasing, which validates the selection and shall
// return to default
//
#[allow(unused_must_use)]
impl State {
    fn enter_selectregion(_ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn exit_selectregion(_ctx: &egui::Context, _frame: &mut eframe::Frame) {}

    fn run_selectregion(ctx: &egui::Context, _frame: &mut eframe::Frame) -> Event {
        _frame.set_maximized(true);
        _frame.set_decorations(false);
        let frame = egui::Frame::default();
        {
            let mut this = frame;
            let fill = Color32::TRANSPARENT;
            this.fill = fill;
            this
        };

        let mut panel = CentralPanel::default();
        panel = panel.frame(frame);

        panel
            .show(ctx, |ui| State::on_panel_shown_selectregion(ui, ctx))
            .inner
    }

    fn on_panel_shown_selectregion(ui: &mut Ui, ctx: &egui::Context) -> Event {
        let (res, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, res.rect.size()),
            res.rect,
        );

        if let Some(selection) = {
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
            ctx.data_mut(|data| {
                data.insert_persisted(egui::Id::new(DATA_SELECTION_ID), selection);
            });

            let shape = RectShape {
                rect: selection,
                rounding: Rounding::default(),
                fill: Color32::TRANSPARENT,
                stroke: Stroke {
                    width: 1.0,
                    color: Color32::RED,
                },
            };

            painter.add(shape);

            return Event::RegionSelectionUpdated;
        }

        if res.drag_released_by(egui::PointerButton::Primary) {
            return Event::RegionSelectionFinished;
        }

        Event::Nothing
    }
}

//
// --- The main App acts as a state machine that defers display code to several states
// --- States are analogous to views
//

pub struct App {
    prev_state: State,
    cur_state: State,
    cur_event: Event,
    prev_event: Event,

    selection: Rect,
}

impl Default for App {
    fn default() -> Self {
        Self {
            cur_state: State::Default,
            prev_state: State::None,
            cur_event: Event::Nothing,
            prev_event: Event::Nothing,

            selection: egui::Rect::NOTHING,
        }
    }
}

#[allow(unused_must_use)]
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        println!("Update state: {:?}", self.cur_state);

        // Handle state machine logic
        self.cur_event = self.cur_state.run(ctx, _frame);
        let new_state = self.get_next_state();

        if self.cur_state != new_state {
            self.cur_state.exit(ctx, _frame);
            new_state.enter(ctx, _frame);

            println!("Exit: {:?}, Enter: {:?}", self.cur_state, new_state);

            self.prev_state = self.cur_state;
            self.cur_state = new_state;
        }

        self.prev_event = self.cur_event;
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0., 0., 0., 0.5]
    }
}
