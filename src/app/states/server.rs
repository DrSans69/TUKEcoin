use crate::app::states::{
    Action, AppState, Context, Event, Frame, Menu, MenuItem, MenuState, State,
};

pub struct StateServer {
    menu: Menu,
}

impl StateServer {
    pub fn new() -> Self {
        let list_items = vec![
            MenuItem::new("Print".into(), Action::Print("PICKLE RICK!".to_string())),
            MenuItem::new("Back".into(), Action::ChangeState(State::Idle)),
        ];
        Self {
            menu: Menu::new(list_items),
        }
    }
}

impl MenuState for StateServer {
    fn get_title(&self) -> &str {
        "Server"
    }

    fn get_menu(&mut self) -> &mut Menu {
        &mut self.menu
    }
}

impl AppState for StateServer {
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
