mod client;
mod idle;
mod server;

use crate::app::{
    components::{Menu, MenuItem},
    Action, Context, State,
};
use client::StateClient;
use idle::StateIdle;
use server::StateServer;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint::Fill, Layout},
    style::Stylize,
    text::Line,
    widgets::Block,
    Frame,
};

// Traits
pub trait AppState {
    fn on_enter(&mut self);
    fn on_exit(&mut self);
    fn draw(&mut self, frame: &mut Frame, ctx: &mut Context);
    fn handle_events(&mut self, event: &Event) -> Option<Action>;
}

trait MenuState: AppState {
    fn get_title(&self) -> &str;
    fn get_menu(&mut self) -> &mut Menu;

    fn get_hints(&self) -> Line<'static> {
        Line::from(vec![
            " Navigation ".into(),
            "<↑↓>".gray(),
            " Select ".into(),
            "<Enter>".gray(),
            " Quit ".into(),
            "<q>".gray(),
            " ".into(),
        ])
        .centered()
    }

    fn draw_menu(&mut self, frame: &mut Frame, ctx: &mut Context) {
        let title = self.get_title().to_string();
        let hints = self.get_hints();
        let menu = self.get_menu();

        let main_block = Block::bordered().title(title.bold()).title_bottom(hints);
        let inner_area = main_block.inner(frame.area());
        frame.render_widget(main_block, frame.area());

        let horizontal_layout = Layout::horizontal([Fill(1), Fill(1)]);
        let [left_area, right_area] = horizontal_layout.areas(inner_area);

        menu.draw(frame, left_area);

        ctx.output.draw(frame, right_area);
    }

    fn handle_quit_and_menu(&mut self, event: &Event) -> Option<Action> {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Some(Action::Quit);
            }
        }

        self.get_menu().handle_events(event)
    }

    fn select_first(&mut self) {
        let menu = self.get_menu();
        menu.state.select(Some(0));
    }
}

// Manager
pub struct StateManager {
    state: State,
    idle_state: StateIdle,
    client_state: StateClient,
    server_state: StateServer,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state: State::Idle,
            idle_state: StateIdle::new(),
            client_state: StateClient::new(),
            server_state: StateServer::new(),
        }
    }

    pub fn current_state(&mut self) -> &mut dyn AppState {
        match self.state {
            State::Idle => &mut self.idle_state,
            State::Client => &mut self.client_state,
            State::Server => &mut self.server_state,
        }
    }

    pub fn transition(&mut self, new_state: State) {
        self.current_state().on_exit();
        self.state = new_state;
        self.current_state().on_enter();
    }
}
