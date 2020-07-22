use tui::backend::Backend;
use tui::terminal::Terminal;

use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Text, Block, Borders, List, ListState, Paragraph};

// draws the layout to the terminal
// this function gets called everytime something changes
// so everything gets redrawn
pub fn draw<B: Backend>(terminal: &mut Terminal<B>, scroll_views: Vec<(String, bool)>) {
    terminal.draw(|mut f| {
        // the search bar will take up 10%
        // the rest goes to the list view
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(90)
                ].as_ref()
            )
            .split(f.size());

        // the search bar
        f.render_widget(
            Paragraph::new(
                    vec![Text::raw("It works!")].iter()
                )
                .block(
                    Block::default().borders(Borders::ALL).title(" Search ")
                )
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left)
                .wrap(true),
            chunks[0]
        );

        // the scrollable list view
        f.render_stateful_widget(
            List::new(
                    scroll_views
                        .iter()
                        .map(|(i, _)| Text::raw(i.clone()))
                )
                .block(
                    Block::default().borders(Borders::ALL).title(" List ")
                )
                .highlight_style(
                    Style::default().modifier(Modifier::BOLD)
                )
                .highlight_symbol(">"),
            chunks[1],
            &mut ListState::default()
        );
    });
}
