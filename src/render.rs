use super::widgets::Widget;
use super::widgets::{ListWidget, SearchWidget, Selectable};

use tui::backend::Backend;
use tui::terminal::Terminal;

use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListState, Paragraph};

// draws the layout to the terminal
// this function gets called everytime something changes
// so everything gets redrawn
pub fn draw<B: Backend>(terminal: &mut Terminal<B>, list_widget: &ListWidget, search_widget: &SearchWidget, selected: &Selectable) {
    terminal.draw(|mut f| {
        // the search bar will take up 10%
        // the rest goes to the list view
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Percentage(90)
                ].as_ref()
            )
            .split(f.size());

        // the search bar
        let search_widget_content = search_widget.display();
        let mut search_widget_paragraph = Paragraph::new(search_widget_content.iter())
            .block(
                Block::default().borders(Borders::ALL).title(" Search ")
            )
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(true);


        // the scrollable list view
        let mut list_widget_state = ListState::default();
       
        let list_widget_content = list_widget.display();
        let mut list_widget_paragraph = List::new(list_widget_content.into_iter())
            .block(
                Block::default().borders(Borders::ALL).title(" List ")
            )
            .highlight_style(
                Style::default().modifier(Modifier::BOLD)
            )
            .highlight_symbol("> ");

        // highlight the current selected widget
        match selected {
            Selectable::Search => {
                search_widget_paragraph = search_widget_paragraph.block(
                    Block::default().title(" Search ")
                        .borders(Borders::ALL).border_style(Style::default().fg(Color::Red))
                );
            }
            Selectable::List => {
                list_widget_paragraph = list_widget_paragraph.block(
                    Block::default().title(" List ")
                        .borders(Borders::ALL).border_style(Style::default().fg(Color::Red))
                );
                list_widget_state.select(Some(list_widget.selected));
            }
        }

        // render all the widgets
        f.render_widget(search_widget_paragraph.clone(), chunks[0]);
        f.render_stateful_widget(list_widget_paragraph.clone(), chunks[1], &mut list_widget_state);
    }).unwrap();
}
