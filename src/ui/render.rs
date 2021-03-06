use super::config;
use super::widgets::{ContentWidget, InfoWidget, SearchWidget, Selectable};
use super::widgets::{ListWidget, ParagraphWidget};

use tui::backend::Backend;
use tui::terminal::Terminal;

use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListState, Paragraph, Wrap};

// Draws the layout to the terminal
// This function gets called everytime something changes
// so everything gets redrawn
pub fn draw<B: Backend>(
    terminal: &mut Terminal<B>,
    list_widget: &ContentWidget,
    search_widget: &SearchWidget,
    info_widget: &InfoWidget,
    selected: &Selectable,
    config: &config::Config,
) {
    // Create default values with the
    // priveded configurations in the Config struct

    // Create an rgb color out of an array
    let color_rgb = |arr: config::Color| {
        if let Some(af) = arr.fg {
            if let Some(ab) = arr.bg {
                Style::default()
                    .fg(Color::Rgb(af[0], af[1], af[2]))
                    .bg(Color::Rgb(ab[0], ab[1], ab[2]))
            } else {
                Style::default().fg(Color::Rgb(af[0], af[1], af[2]))
            }
        } else if let Some(ab) = arr.bg {
            Style::default().bg(Color::Rgb(ab[0], ab[1], ab[2]))
        } else {
            Style::default()
        }
    };

    // blocks = config.theme
    let block_selected = || {
        Block::default()
            .borders(Borders::ALL)
            .border_style(color_rgb(config.theme.selected.clone()))
    };

    let block_default = || {
        Block::default()
            .borders(Borders::ALL)
            .border_style(color_rgb(config.theme.default.clone()))
    };

    terminal
        .draw(|f| {
            // The search bar will take up 10%
            // The rest goes to the list view
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Length(3), Constraint::Percentage(90)].as_ref())
                .split(f.size());
            // Chunk used indirectly to create info_chunk
            let info_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints([Constraint::Min(10), Constraint::Length(10)].as_ref())
                .split(chunks[0]);

            // The search bar
            let search_widget_content = search_widget.display(config.lame, String::new());
            let search_widget_title =
                search_widget.get_title(config.lame, config.prefixes.search.clone());
            let search_widget_paragraph = Paragraph::new(search_widget_content)
                .block({
                    match selected {
                        Selectable::Search => block_selected().title(search_widget_title.as_str()),
                        _ => block_default().title(search_widget_title.as_str()),
                    }
                })
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: false });

            // The info widget
            let info_widget_content = info_widget.display(config.lame, String::new());
            let info_widget_paragraph = Paragraph::new(info_widget_content)
                .block(block_default())
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Right)
                .wrap(Wrap { trim: false });

            // The scrollable list view
            let mut list_widget_state = ListState::default();
            let list_widget_content =
                list_widget.display(f.size(), config.lame, config.prefixes.folder.clone());
            let list_widget_title =
                list_widget.get_title(config.lame, config.prefixes.list.clone());
            let list_widget_list = List::new(list_widget_content)
                .block({
                    match selected {
                        Selectable::List => {
                            list_widget_state.select(Some(list_widget.get_selected(f.size())));
                            block_selected().title(list_widget_title.as_str())
                        }
                        _ => block_default().title(list_widget_title.as_str()),
                    }
                })
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(config.selector.as_str());

            // Render all the widgets
            f.render_widget(search_widget_paragraph.clone(), info_chunk[0]);
            f.render_widget(info_widget_paragraph.clone(), info_chunk[1]);
            f.render_stateful_widget(list_widget_list.clone(), chunks[1], &mut list_widget_state);
        })
        .unwrap();
}
