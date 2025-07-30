use std::collections::LinkedList;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint::Fill, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, List, ListState},
    DefaultTerminal, Frame,
};

// use tokio::{sync::mpsc, task::Id};

// consts
const MAX_OUTPUT_LENGTH: usize = 128;

// Enums
#[derive(Clone, Copy)]
enum State {
    Idle,
    Client,
    Server,
}

#[derive(Clone)]
enum Action {
    Quit,
    ChangeState(State),
    Print(String),
}

// Output
struct Output {
    list: LinkedList<String>,
    max_length: usize,
}

impl Output {
    fn new() -> Self {
        Self {
            list: LinkedList::new(),
            max_length: MAX_OUTPUT_LENGTH,
        }
    }

    fn add(&mut self, s: String) {
        self.list.push_back(s);
        if self.list.len() > self.max_length {
            self.list.pop_front();
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title("Output".bold());
        let inner_area = block.inner(area);

        let available_lines = inner_area.height as usize;

        let items: Vec<&str> = self
            .list
            .iter()
            .rev()
            .take(available_lines)
            .rev()
            .map(|s| s.as_str())
            .collect();

        let list = List::new(items)
            .block(block) // Use the same block
            .style(Style::new().white());

        frame.render_widget(list, area);
    }
}

// Menu
struct MenuItem {
    text: String,
    action: Action,
}

impl MenuItem {
    fn new(text: String, action: Action) -> Self {
        Self { text, action }
    }
}

struct Menu {
    items: Vec<MenuItem>,
    state: ListState,
}

impl Menu {
    fn new(items: Vec<MenuItem>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self { items, state }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<&str> = self.items.iter().map(|item| item.text.as_str()).collect();

        let list = List::new(items)
            .block(Block::bordered().title("Menu".bold()))
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        frame.render_stateful_widget(list, area, &mut self.state);
    }

    fn handle_events(&mut self, event: &Event) -> Option<Action> {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Up => {
                    self.state.select_previous();
                }
                KeyCode::Down => {
                    self.state.select_next();
                }
                KeyCode::Enter => {
                    let selected = self.state.selected()?;
                    let item = self.items.get(selected)?;

                    return Some(item.action.clone());
                }
                _ => {}
            },
            _ => {}
        }
        None
    }
}

// Traits
trait AppState {
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

/* States Impl */
// Idle
struct StateIdle {
    menu: Menu,
}

impl StateIdle {
    fn new() -> Self {
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

// Client
struct StateClient {
    menu: Menu,
}

impl StateClient {
    fn new() -> Self {
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

// Server
struct StateServer {
    menu: Menu,
}

impl StateServer {
    fn new() -> Self {
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

// Managers
struct StateManager {
    state: State,
    idle_state: StateIdle,
    client_state: StateClient,
    server_state: StateServer,
}

impl StateManager {
    fn new() -> Self {
        Self {
            state: State::Idle,
            idle_state: StateIdle::new(),
            client_state: StateClient::new(),
            server_state: StateServer::new(),
        }
    }

    fn current_state(&mut self) -> &mut dyn AppState {
        match self.state {
            State::Idle => &mut self.idle_state,
            State::Client => &mut self.client_state,
            State::Server => &mut self.server_state,
        }
    }

    fn transition(&mut self, new_state: State) {
        self.current_state().on_exit();
        self.state = new_state;
        self.current_state().on_enter();
    }
}

// Context
struct Context {
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
struct App {
    terminal: DefaultTerminal,
    state_manager: StateManager,
    context: Context,
    exit: bool,
}

impl App {
    fn new(terminal: DefaultTerminal) -> Self {
        Self {
            terminal,
            state_manager: StateManager::new(),
            context: Context::new(),
            exit: false,
        }
    }

    fn run(&mut self) {
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

#[tokio::main]
async fn main() {
    let terminal = ratatui::init();

    let mut app: App = App::new(terminal);
    app.run();

    ratatui::restore();
}
