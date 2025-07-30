mod components;
mod states;

use components::Output;
use states::StateManager;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

// use tokio::{sync::mpsc, task::Id};

// Basics
#[derive(Clone, Copy)]
pub enum State {
    Idle,
    Client,
    Server,
}

#[derive(Clone)]
pub enum Action {
    Quit,
    ChangeState(State),
    Print(String),
}

pub struct Context {
    output: Output,
}

impl Context {
    fn new() -> Self {
        Self {
            output: Output::new(),
        }
    }
}

// App
pub struct App {
    terminal: DefaultTerminal,
    state_manager: StateManager,
    context: Context,
    exit: bool,
}

impl App {
    pub fn new(terminal: DefaultTerminal) -> Self {
        Self {
            terminal,
            state_manager: StateManager::new(),
            context: Context::new(),
            exit: false,
        }
    }

    pub fn run(&mut self) {
        while !self.exit {
            self.run_routine()
        }
    }

    fn run_routine(&mut self) {
        let state = self.state_manager.current_state();
        let terminal = &mut self.terminal;

        terminal
            .draw(|frame| state.draw(frame, &mut self.context))
            .unwrap();

        let event = event::read().unwrap();

        let action = state.handle_events(&event);
        if action.is_some() {
            self.handle_actions(action.unwrap());
        }

        let action_global = self.handle_global_events(&event);
        if action_global.is_some() {
            self.handle_actions(action_global.unwrap());
        }
    }

    fn handle_global_events(&self, event: &Event) -> Option<Action> {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('c')
                    if key
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::CONTROL) =>
                {
                    return Some(Action::Quit);
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn handle_actions(&mut self, action: Action) {
        match action {
            Action::Quit => {
                self.exit = true;
            }
            Action::ChangeState(new_state) => {
                self.state_manager.transition(new_state);
            }
            Action::Print(s) => {
                self.context.output.add(s);
            }
        }
    }
}
