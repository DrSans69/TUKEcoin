mod components;
mod states;

use crate::network::{run_client, run_server};

use components::Output;
use states::{State, StateManager};

use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use std::time::Duration;
use tokio::sync::mpsc;

// Basics

#[derive(Clone, Debug)]
pub enum Action {
    Quit,
    ChangeState(State),
    Print(String),
    StartNetwork(u16),
    StopNetwork,
    NetworkMessage(String),
}

pub struct Context {
    output: Output,
    pub action_sender: mpsc::UnboundedSender<Action>,
    netwrok_port: u16,
}

impl Context {
    fn new(action_sender: mpsc::UnboundedSender<Action>) -> Self {
        Self {
            output: Output::new(),
            action_sender,
            netwrok_port: 0,
        }
    }
}

// App
pub struct App {
    terminal: DefaultTerminal,
    state_manager: StateManager,
    ctx: Context,
    exit: bool,
    action_receiver: mpsc::UnboundedReceiver<Action>,
    network_handle: Option<tokio::task::JoinHandle<()>>,
}

impl App {
    pub fn new(terminal: DefaultTerminal) -> Self {
        let (action_sender, action_receiver) = mpsc::unbounded_channel();

        Self {
            terminal,
            state_manager: StateManager::new(),
            ctx: Context::new(action_sender),
            exit: false,
            action_receiver,
            network_handle: None,
        }
    }

    fn print<S: AsRef<str>>(&mut self, s: S) {
        self.ctx.output.add(s.as_ref().to_string());
    }

    fn action(&mut self, action: Action) {
        if let Err(e) = self.ctx.action_sender.send(action) {
            self.print(format!("Action Error {}", e));
        }
    }

    pub fn run(&mut self) {
        while !self.exit {
            self.run_routine()
        }

        self.stop_network_task();
    }

    fn run_routine(&mut self) {
        while let Ok(action) = self.action_receiver.try_recv() {
            self.handle_actions(action);
        }

        let state = self.state_manager.current_state();

        let terminal = &mut self.terminal;

        terminal
            .draw(|frame| state.draw(frame, &mut self.ctx))
            .unwrap();

        // error unsafe
        if !poll(Duration::from_millis(100)).unwrap() {
            return;
        }

        let event = read().unwrap();

        state.handle_events(&event, &mut self.ctx);

        self.handle_global_events(&event);
    }

    fn handle_global_events(&mut self, event: &Event) {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('c')
                    if key
                        .modifiers
                        .contains(crossterm::event::KeyModifiers::CONTROL) =>
                {
                    self.action(Action::Quit);
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn handle_actions(&mut self, action: Action) {
        self.print(format!("{:?}", action));
        match action {
            Action::Quit => {
                self.exit = true;
            }
            Action::ChangeState(new_state) => {
                self.state_manager.transition(new_state, &mut self.ctx);
            }
            Action::Print(s) => {
                self.print(s);
            }
            Action::NetworkMessage(msg) => {
                self.print(format!("[NET] {}", msg));
            }
            Action::StartNetwork(port) => {
                if port == 0 {
                    self.start_client();
                } else {
                    self.start_server();
                }
            }
            Action::StopNetwork => {
                self.stop_network_task();
            }
        }
    }

    fn start_server(&mut self) {
        let sender = self.ctx.action_sender.clone();

        if self.network_handle.is_some() {
            self.print("Some network process is already started");
            return;
        }

        let handle = tokio::spawn(async move {
            if let Err(e) = run_server(sender.clone()).await {
                let _ = sender.send(Action::NetworkMessage(format!("Server error: {}", e)));
            }
        });

        self.network_handle = Some(handle);

        self.print("Server started");
    }

    fn start_client(&mut self) {
        let sender = self.ctx.action_sender.clone();

        if self.network_handle.is_some() {
            self.print("Some network process is already started");
            return;
        }

        let handle = tokio::spawn(async move {
            if let Err(e) = run_client(sender.clone()).await {
                let _ = sender.send(Action::NetworkMessage(format!("Server error: {}", e)));
            }
        });

        self.network_handle = Some(handle);

        self.print("Client started");
    }

    fn stop_network_task(&mut self) {
        if let Some(handle) = &self.network_handle {
            handle.abort();

            self.print("Network process stopped");
            self.network_handle = None;
        }
    }
}
