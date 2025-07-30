use crate::app::states::{
    Action, AppState, Context, Event, Frame, Menu, MenuItem, MenuState, State,
};

pub struct StateClient {
    menu: Menu,
}

impl StateClient {
    pub fn new() -> Self {
        let list_items = vec![
            MenuItem::new(
                "Print".into(),
                Action::Print("WUBBA LUBBA DUB DUB!".to_string()),
            ),
            MenuItem::new("Back".into(), Action::ChangeState(State::Idle)),
        ];
        Self {
            menu: Menu::new(list_items),
        }
    }
}

impl MenuState for StateClient {
    fn get_title(&self) -> &str {
        "Client"
    }

    fn get_menu(&mut self) -> &mut Menu {
        &mut self.menu
    }
}

impl AppState for StateClient {
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
