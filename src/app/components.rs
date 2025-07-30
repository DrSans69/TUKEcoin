use crate::app::Action;

use std::collections::LinkedList;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, List, ListState},
    Frame,
};

const MAX_OUTPUT_LENGTH: usize = 128;

// Output
pub struct Output {
    list: LinkedList<String>,
    max_length: usize,
}

impl Output {
    pub fn new() -> Self {
        Self {
            list: LinkedList::new(),
            max_length: MAX_OUTPUT_LENGTH,
        }
    }

    pub fn add(&mut self, s: String) {
        self.list.push_back(s);
        if self.list.len() > self.max_length {
            self.list.pop_front();
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
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
pub struct MenuItem {
    text: String,
    action: Action,
}

impl MenuItem {
    pub fn new(text: String, action: Action) -> Self {
        Self { text, action }
    }
}

pub struct Menu {
    items: Vec<MenuItem>,
    pub state: ListState,
}

impl Menu {
    pub fn new(items: Vec<MenuItem>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self { items, state }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<&str> = self.items.iter().map(|item| item.text.as_str()).collect();

        let list = List::new(items)
            .block(Block::bordered().title("Menu".bold()))
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        frame.render_stateful_widget(list, area, &mut self.state);
    }

    pub fn handle_events(&mut self, event: &Event) -> Option<Action> {
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
