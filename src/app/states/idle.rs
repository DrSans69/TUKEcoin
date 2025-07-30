use crate::app::states::{
    Action, AppState, Context, Event, Frame, Menu, MenuItem, MenuState, State,
};

pub struct StateIdle {
    menu: Menu,
}

impl StateIdle {
    pub fn new() -> Self {
        let list_items = vec![
            MenuItem::new("Client".into(), Action::ChangeState(State::Client)),
            MenuItem::new("Server".into(), Action::ChangeState(State::Server)),
            MenuItem::new("Exit".into(), Action::Quit),
        ];
        Self {
            menu: Menu::new(list_items),
        }
    }
}

impl MenuState for StateIdle {
    fn get_title(&self) -> &str {
        "Idle"
    }

    fn get_menu(&mut self) -> &mut Menu {
        &mut self.menu
    }
}

impl AppState for StateIdle {
    fn on_enter(&mut self) {
        self.select_first();
    }

    fn on_exit(&mut self) {}

    fn draw(&mut self, frame: &mut Frame, ctx: &mut Context) {
        self.draw_menu(frame, ctx);
    }

    fn handle_events(&mut self, event: &Event) -> Option<Action> {
        self.handle_quit_and_menu(event)
    }
}
