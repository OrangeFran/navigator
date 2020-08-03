extern crate clap;
use clap::{Arg, App};

mod widgets;
mod render;
mod config;

use widgets::Direction;
use widgets::{Selectable, ListWidget, SearchWidget};

use std::fs::File;
use std::io::Read;
use std::io::{stdin, stdout};

use tui::terminal::Terminal;
use tui::backend::TermionBackend;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{RawTerminal, IntoRawMode};

// set up the terminal -> into raw mode
fn setup() -> Terminal<TermionBackend<RawTerminal<std::io::Stdout>>> {
    let raw = stdout().into_raw_mode().expect("Failed to put the terminal into raw mode");
    let backend = TermionBackend::new(raw);

    let mut terminal = Terminal::new(backend).expect("Failed to create the terminal");

    terminal.hide_cursor().expect("Failed to hide the cursor");
    terminal.clear().expect("Failed to clear the terminal");

    terminal
}

fn main() {
    // setup the cli app
    let matches = App::new("navigator")
        .version("0.1")
        .author("Finn H.")
        .about("Navigate through string-based structures with ease!")
        .arg(Arg::with_name("INPUT")
             .help("Sets the string to use")
             .required_unless("stdin"))
        .arg(Arg::with_name("stdin")
             .short("s")
             .long("stdin")
             .help("Gets input over stdin"))
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .takes_value(true)
             .help("Sets the path to the config file"))
        .get_matches();

    let stdin = stdin();
    let mut input = String::new();
    match matches.occurrences_of("stdin") {
        0 => {
            input = matches.value_of("INPUT")
                .expect("No INPUT provided").to_string();
        }
        // try to use INTPUT if defined
        // else print an error
        _ => {
            let mut handle = stdin.lock(); 
            handle.read_to_string(&mut input).expect("Failed to receive from stdin");
        }
    }

    // open input file and read to string
    let mut config = String::new();
    if let Some(c) = matches.value_of("config") {
        File::open(c)
            .expect("Failed to open config")
            .read_to_string(&mut config)
            .expect("Failed to read config");
    }

    let config = config::read_config(config.as_str());

    let mut terminal = setup();
    let mut selected = Selectable::List;
    let mut search_widget = SearchWidget::new();
    let mut list_widget = ListWidget::from_string(input);

    // draw the layout for the first time
    render::draw(&mut terminal, &list_widget, &search_widget, &selected, &config);

    let handle = stdin.lock();
    // wait for input events
    for event in handle.events() {
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

                    terminal.hide_cursor().expect("Failed to hide cursor");
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

                    terminal.hide_cursor().expect("Failed to hide cursor");
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
                // expand an element
                Event::Key(Key::Right) | Event::Key(Key::Char('l')) => {
                    list_widget.expand();
                }
                // expand an element
                Event::Key(Key::Left) | Event::Key(Key::Char('h')) => {
                    list_widget.back();
                }
                // switch to search widget
                Event::Key(Key::Char('/')) => {
                    selected = Selectable::Search;

                    terminal.show_cursor().expect("Failed to show cursor");
                }
                // print out the selected element to stdout
                Event::Key(Key::Char('\n')) => {
                    terminal.clear().expect("Failed to clear the terminal");
                    // raw.suspend_raw_mode();
                    println!("{}", list_widget.get_name());
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
        render::draw(&mut terminal, &list_widget, &search_widget, &selected, &config);
    }
}
