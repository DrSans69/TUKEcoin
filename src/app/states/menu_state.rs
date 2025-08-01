use super::{Action, AppState, Context, Menu};

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint::Fill, Layout},
    style::Stylize,
    text::Line,
    widgets::Block,
    Frame,
};

pub trait MenuState: AppState {
    fn get_title(&self) -> &str;
    fn get_menu(&mut self) -> &mut Menu;

    fn hints(&self) -> Line<'static> {
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
        let hints = self.hints();
        let menu = self.get_menu();

        let main_block = Block::bordered().title(title.bold()).title_bottom(hints);
        let inner_area = main_block.inner(frame.area());
        frame.render_widget(main_block, frame.area());

        let horizontal_layout = Layout::horizontal([Fill(1), Fill(3)]);
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
