mod idle;
mod menu_state;
mod network;
mod transacions;

use crate::app::{
    components::{Menu, MenuItem},
    Action, Context,
};
use idle::IdleState;
use menu_state::MenuState;
use network::NetworkState;
use transacions::TransactionsState;

use crossterm::event::Event;
use ratatui::Frame;

#[derive(Clone, Copy, Debug)]
pub enum State {
    Idle,
    Network(u16),
    Transactions,
    Mining,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (State::Idle, State::Idle) => true,
            (State::Network(_), State::Network(_)) => true,
            (State::Transactions, State::Transactions) => true,
            (State::Mining, State::Mining) => true,
            _ => false,
        }
    }
}

// Traits
pub trait AppState {
    fn get_parent(&self) -> Option<State>;

    fn on_enter(&mut self, ctx: &mut Context);
    fn on_exit(&mut self, ctx: &mut Context);
    fn draw(&mut self, frame: &mut Frame, ctx: &mut Context);
    fn handle_events(&mut self, event: &Event, ctx: &mut Context);
}

// Manager
pub struct StateManager {
    state: State,
    idle: IdleState,
    network: NetworkState,
    transactions: TransactionsState,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state: State::Idle,
            idle: IdleState::new(),
            network: NetworkState::new(),
            transactions: TransactionsState::new(),
        }
    }

    fn get_state(&mut self, state: State) -> &mut dyn AppState {
        match state {
            State::Idle => &mut self.idle,
            State::Network(_) => &mut self.network,
            State::Transactions => &mut self.transactions,
            _ => &mut self.idle,
        }
    }

    pub fn current_state(&mut self) -> &mut dyn AppState {
        self.get_state(self.state)
    }

    fn build_path_to_root(&mut self, mut state: State) -> Vec<State> {
        let mut path = vec![state.clone()];
        while let Some(parent) = self.get_state(state).get_parent() {
            path.push(parent.clone());
            state = parent;
        }
        path
    }

    pub fn transition(&mut self, new_state: State, ctx: &mut Context) {
        match new_state {
            State::Network(port) => {
                ctx.netwrok_port = port;
            }
            _ => {}
        }

        let mut current_path = self.build_path_to_root(self.state);
        let mut target_path = self.build_path_to_root(new_state);

        current_path.reverse();
        target_path.reverse();

        let mut lca_index = 0;
        while lca_index < current_path.len()
            && lca_index < target_path.len()
            && current_path[lca_index] == target_path[lca_index]
        {
            lca_index += 1;
        }

        for state in current_path.iter().skip(lca_index).rev() {
            self.get_state(*state).on_exit(ctx);
        }

        for state in target_path.iter().skip(lca_index) {
            self.get_state(*state).on_enter(ctx);
        }

        self.state = new_state;
    }
}
