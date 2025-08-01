use super::{Action, AppState, Context, Menu, MenuItem, MenuState, State};

use crossterm::event::Event;
use ratatui::Frame;

pub struct NetworkState {
    menu: Menu,
}

impl NetworkState {
    pub fn new() -> Self {
        Self {
            menu: Menu::new(vec![
                MenuItem::new(
                    "Make a transaction".to_string(),
                    Action::ChangeState(State::Transactions),
                ),
                MenuItem::new("Exit".to_string(), Action::ChangeState(State::Idle)),
            ]),
        }
    }
}

impl AppState for NetworkState {
    fn get_parent(&self) -> Option<State> {
        None
    }

    fn on_enter(&mut self, ctx: &mut Context) {
        let _ = ctx
            .action_sender
            .send(Action::StartNetwork(ctx.netwrok_port));
    }

    fn on_exit(&mut self, ctx: &mut Context) {
        let _ = ctx.action_sender.send(Action::StopNetwork);
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

impl MenuState for NetworkState {
    fn get_title(&self) -> &str {
        "Running"
    }
    fn get_menu(&mut self) -> &mut Menu {
        &mut self.menu
    }
}
