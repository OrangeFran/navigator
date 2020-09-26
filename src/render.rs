use super::widgets::Widget;
use super::widgets::{ListWidget, SearchWidget, InfoWidget, Selectable};
use super::config;

use tui::backend::Backend;
use tui::terminal::Terminal;

use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListState, Paragraph};

// draws the layout to the terminal
// this function gets called everytime something changes
// so everything gets redrawn
pub fn draw<B: Backend>(
    terminal: &mut Terminal<B>, 
    list_widget: &ListWidget, search_widget: &SearchWidget, info_widget: &InfoWidget,
    selected: &Selectable, config: &config::Config
) {
    // create default values with the
    // priveded configurations in the Config struct
   
    // create an rgb color out of an array
    let color_rgb = |arr: config::Color| {
        if let Some(af) = arr.fg {
            if let Some(ab) = arr.bg {
                Style::default()
                    .fg(Color::Rgb(af[0], af[1], af[2]))
                    .bg(Color::Rgb(ab[0], ab[1], ab[2]))
            } else {
                Style::default()
                    .fg(Color::Rgb(af[0], af[1], af[2]))
            }
        } else if let Some(ab) = arr.bg {
            Style::default()
                .bg(Color::Rgb(ab[0], ab[1], ab[2]))
        } else {
            Style::default()
        }
    };
    
    // blocks = config.theme
    let block_selected = || {
        Block::default()
            .borders(Borders::ALL)
            .border_style(
                color_rgb(config.theme.selected.clone())
            )
    };

    let block_default = || {
        Block::default()
            .borders(Borders::ALL)
            .border_style(
                color_rgb(config.theme.default.clone())
            )
    };

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
        // chunk used indirectly to create info_chunk
        let info_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Min(10),
                    Constraint::Length(5)
                ].as_ref()
            )
            .split(chunks[0]);

        // the search bar
        let search_widget_content = search_widget.display(config.lame, String::new());
        let search_widget_title = search_widget.get_title(config.lame, config.prefixes.search.clone());
        let search_widget_paragraph = Paragraph::new(search_widget_content.iter())
            .block({
                match selected {
                    Selectable::Search => block_selected().title(search_widget_title.as_str()),
                    _ => block_default().title(search_widget_title.as_str())
                }
            })
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(true);

        // the info widget
        let info_widget_content = info_widget.display(config.lame, String::new());
        let info_widget_title = info_widget.get_title(config.lame, String::new());
        let info_widget_paragraph = Paragraph::new(info_widget_content.iter())
            .block(block_default())
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .wrap(true);
        // ... continue implementing the info widget

        // the scrollable list view
        let mut list_widget_state = ListState::default();
        let list_widget_content = list_widget.display(config.lame, config.prefixes.folder.clone());
        let list_widget_title = list_widget.get_title(config.lame, config.prefixes.list.clone());
        let list_widget_list = List::new(list_widget_content.into_iter())
            .block({
                match selected {
                    Selectable::List => {
                        list_widget_state.select(Some(list_widget.selected));
                        block_selected().title(list_widget_title.as_str())
                    }
                    _ => block_default().title(list_widget_title.as_str())
                }
            })
            .highlight_style(Style::default().modifier(Modifier::BOLD))
            .highlight_symbol(config.selector.as_str());

        // // highlight the current selected widget
        // match selected {
        //     Selectable::Search => {
        //         search_widget_paragraph = search_widget_paragraph.block(
        //             block_selected(" 🔍 Search ")
        //         );
        //     }
        //     Selectable::List => {
        //         list_widget_list = list_widget_list.block(
        //             block_selected(list_widget_title.as_str())
        //         );
        //         list_widget_state.select(Some(list_widget.selected));
        //     }
        // }

        // render all the widgets
        f.render_widget(search_widget_paragraph.clone(), info_chunk[0]);
        f.render_widget(info_widget_paragraph.clone(), info_chunk[1]);
        f.render_stateful_widget(list_widget_list.clone(), chunks[1], &mut list_widget_state);
    }).unwrap();
}
