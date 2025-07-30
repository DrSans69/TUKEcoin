mod app;

use app::App;

#[tokio::main]
async fn main() {
    let terminal = ratatui::init();

    let mut app: App = App::new(terminal);
    app.run();

    ratatui::restore();
}
