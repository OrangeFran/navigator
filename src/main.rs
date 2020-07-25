mod widgets;
mod render;

use widgets::{Type, Direction};
use widgets::{Selectable, ListWidget, SearchWidget};

use std::io::{stdin, stdout};

use tui::terminal::Terminal;
use tui::backend::TermionBackend;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    // set up the terminal -> into raw mode
    let raw = stdout().into_raw_mode().expect("Failed to put the terminal into raw mode");
    let backend = TermionBackend::new(raw);
    let mut terminal = Terminal::new(backend).expect("Failed to create the terminal");

    terminal.hide_cursor().expect("Failed to hide the cursor");
    terminal.clear().expect("Failed to clear the terminal");
   
    let mut selected = Selectable::List;

    let mut search_widget = SearchWidget::new();
    let mut list_widget = ListWidget::new(Type::Folder(
        vec![("Hallo".to_string(), Type::Single), ("Hallo".to_string(), Type::Single)]
    ));

    // draw the layout for the first time
    render::draw(&mut terminal, &list_widget, &search_widget, &selected);

    // wait for input events
    for event in stdin().events() {
        // if the program failed
        // to get the event, just continue
        if event.is_err() {
            continue;
        }
    
        match selected {
            Selectable::Search => match event.unwrap() {
                // apply the search
                // must go befor Key::Char(c)
                Event::Key(Key::Char('\n')) => {
                    list_widget.apply_search(search_widget.get_content());
                    selected = Selectable::List;
                }
                // add the char to the search
                Event::Key(Key::Char(c)) => {
                    search_widget.add(c);
                }
                // remove the last char from the search
                Event::Key(Key::Backspace) => {
                    search_widget.pop();
                }
                // switch back to the list view
                Event::Key(Key::Esc) => {
                    selected = Selectable::List;
                }

                _ => {}
            }
            Selectable::List => match event.unwrap() {
                // move up/down/left/right
                // with the arrow or vim keys
                Event::Key(Key::Up) | Event::Key(Key::Char('k')) => {
                    list_widget.scroll(Direction::Up);
                }
                Event::Key(Key::Down) | Event::Key(Key::Char('j')) => {
                    list_widget.scroll(Direction::Down);
                }
                // switch to search widget
                Event::Key(Key::Char('/')) => {
                    selected = Selectable::Search;
                }
                // print out the selected element to stdout
                Event::Key(Key::Char('\n')) => {
                    terminal.clear().expect("Failed to clear the terminal");
                    // raw.suspend_raw_mode();
                    println!("{}", list_widget.get());
                    break;
                }
                // quit the program
                Event::Key(Key::Char('q')) => {
                    terminal.clear().expect("Failed to clear the terminal");
                    break;
                }

                _ => {}
            }
        }

        // update the tui
        render::draw(&mut terminal, &list_widget, &search_widget, &selected);
    }
}
