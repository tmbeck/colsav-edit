use anyhow::Result;
use colonization_sav::SaveFile;

pub mod app;
pub mod colonies_tab;
pub mod header_tab;
pub mod map_tab;
pub mod nations_tab;
pub mod tabs;
pub mod theme;
pub mod trade_routes_tab;
pub mod tribes_tab;
pub mod units_tab;

pub use app::App;

pub fn run(save: SaveFile, path: String) -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new(save, path);
    loop {
        terminal.draw(|frame| app.draw(frame))?;
        if app.handle_events()? {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}
