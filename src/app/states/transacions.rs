use super::{Action, AppState, Context, Menu, MenuItem, MenuState, State};

use crossterm::event::Event;
use ratatui::Frame;

pub struct TransactionsState {
    menu: Menu,
}

impl TransactionsState {
    pub fn new() -> Self {
        Self {
            menu: Menu::new(vec![
                MenuItem::new(
                    "Send".to_string(),
                    Action::Print("PICKLE RICK!".to_string()),
                ),
                MenuItem::new("Exit".to_string(), Action::ChangeState(State::Network(0))),
            ]),
        }
    }
}

impl AppState for TransactionsState {
    fn get_parent(&self) -> Option<State> {
        Some(State::Network(0))
    }

    fn on_enter(&mut self, _ctx: &mut Context) {}
    fn on_exit(&mut self, _ctx: &mut Context) {
        self.select_first();
    }

    fn draw(&mut self, frame: &mut Frame, ctx: &mut Context) {
        self.draw_menu(frame, ctx);
    }

    fn handle_events(&mut self, event: &Event, ctx: &mut Context) {
        if let Some(action) = self.handle_quit_and_menu(event) {
            let _ = ctx.action_sender.send(action);
        }
    }
}

impl MenuState for TransactionsState {
    fn get_title(&self) -> &str {
        "Transactions"
    }
    fn get_menu(&mut self) -> &mut Menu {
        &mut self.menu
    }
}
