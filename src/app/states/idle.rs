use super::{Action, AppState, Context, Menu, MenuItem, MenuState, State};

use crossterm::event::Event;
use ratatui::Frame;

pub struct IdleState {
    menu: Menu,
}

impl IdleState {
    pub fn new() -> Self {
        Self {
            menu: Menu::new(vec![
                MenuItem::new(
                    "Start as a Client".to_string(),
                    Action::ChangeState(State::Network(0)),
                ),
                MenuItem::new(
                    "Start as a Server".to_string(),
                    Action::ChangeState(State::Network(6969)),
                ),
                MenuItem::new("Exit".to_string(), Action::Quit),
            ]),
        }
    }
}

impl AppState for IdleState {
    fn get_parent(&self) -> Option<State> {
        None
    }
    fn on_enter(&mut self, _ctx: &mut Context) {}
    fn on_exit(&mut self, _ctx: &mut Context) {}
    fn draw(&mut self, frame: &mut Frame, ctx: &mut Context) {
        self.draw_menu(frame, ctx);
    }
    fn handle_events(&mut self, event: &Event, ctx: &mut Context) {
        if let Some(action) = self.handle_quit_and_menu(event) {
            let _ = ctx.action_sender.send(action);
        }
    }
}

impl MenuState for IdleState {
    fn get_title(&self) -> &str {
        "Idle"
    }
    fn get_menu(&mut self) -> &mut Menu {
        &mut self.menu
    }
}
