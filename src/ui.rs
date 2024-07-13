use ratatui::{
    layout::{Alignment, Constraint, Flex},
    prelude::Stylize,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Row, Table},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    //
    let rows = app
        .services
        .iter()
        .map(|service| {
            let status = match service.active {
                true => "running",
                false => "stopped",
            };
            let auto_start = match service.auto_start {
                systemctl::AutoStartStatus::Enabled => "true",
                _ => "false",
            };
            Row::new(vec![
                service.clone().name,
                status.to_string(),
                auto_start.to_string(),
                service.clone().description.unwrap_or("".to_string()),
            ])
        })
        .collect::<Vec<_>>();
    let widths = [
        Constraint::Min(10),
        Constraint::Max(7),
        Constraint::Max(8),
        Constraint::Min(5),
    ];
    let table = Table::default().rows(rows).widths(widths);
    let line = Line::from(vec![
        Span::raw(" systemctui - "),
        "s".white().bold().underlined(),
        Span::raw("tart - "),
        "r".white().bold().underlined(),
        Span::raw("estart - sto"),
        "p".white().bold().underlined(),
        Span::raw(" - "),
        "e".white().bold().underlined(),
        Span::raw("nable - "),
        "d".white().bold().underlined(),
        Span::raw("isable "),
    ]);
    // let title = Text::from(vec![line]);
    frame.render_stateful_widget(
        table
            .block(
                Block::bordered()
                    .title(line)
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .header(
                Row::new(vec!["Service", "Status", "Enabled", "Description"])
                    .style(Style::new().bold().underlined())
                    .bottom_margin(1),
            )
            .flex(Flex::Start)
            .highlight_style(Style::new().black().on_blue())
            .highlight_symbol(">>"),
        frame.size(),
        &mut app.table_state,
    )
}
