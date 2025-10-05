use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(frame.size());

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main_layout[0]);

    // Wallet Info
    let wallet_info = Paragraph::new(format!(
        "Alamat Aktif: {:?}\nSaldo: {} UOMI",
        app.get_active_wallet().address(),
        app.balance
    ))
    .block(Block::default().title("Info Dompet").borders(Borders::ALL));
    frame.render_widget(wallet_info, top_layout[0]);

    // Logs
    let logs: Vec<ListItem> = app.logs.iter().map(|s| ListItem::new(s.clone())).collect();
    let logs_list = List::new(logs)
        .block(Block::default().title("Log Transaksi").borders(Borders::ALL));
    frame.render_widget(logs_list, top_layout[1]);

    // Menu
    let menu_text = "Menu:\n[1] Wrap 0.01 UOMI\n[2] Swap Random\n[3] Add LP\n[4] Unwrap 0.01 WUOMI\n[q] Keluar";
    let menu = Paragraph::new(menu_text)
        .block(Block::default().title("Menu").borders(Borders::ALL));
    frame.render_widget(menu, main_layout[1]);
}
