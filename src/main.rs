mod widgets;
use widgets::draw;

use std::io::{stdin, stdout};

use tui::terminal::Terminal;
use tui::backend::TermionBackend;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    // set up the terminal -> into raw mode
    let stdout = stdout().into_raw_mode().expect("Failed to put the terminal into raw mode");
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to create the terminal");

    terminal.hide_cursor().expect("Failed to hide the cursor");
    terminal.clear().expect("Failed to clear the terminal");
    
    // wait for input events
    for event in stdin().events() {
        // if the program failed
        // to get the event, just continue
        if event.is_err() {
            continue;
        }

        match event.unwrap() {
            // quit the program
            Event::Key(Key::Char('q')) => {
                terminal.clear().expect("Failed to clear the terminal");
                break;
            }

            _ => {}
        } 

        // draw/update the tui
        draw(&mut terminal, vec![("Hallo".to_string(), true)])
    }
}
