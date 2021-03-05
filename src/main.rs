mod tests;
mod ui;
mod util;

use ui::{ContentWidget, Direction, InfoWidget, SearchWidget, Selectable};
use util::FileLogger;

use std::fs::File;
use std::io::{stderr, stdin, stdout};
use std::io::{Read, Write};
use std::path::Path;

use clap::{App, Arg};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::terminal::Terminal;

fn main() {
    // Setup the cli app
    let matches = App::new("navigator")
        .version("0.1")
        .author("Finn H.")
        .about("A simply tui-based fuzzy finder")
        .arg(Arg::with_name("INPUT").help("Specifies input string (reads from stdin by default)"))
        .arg(
            Arg::with_name("separator")
                .short("s")
                .long("separator")
                .value_name("SEPARATOR")
                .takes_value(true)
                .help("Separates level with SEPARATOR"),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .takes_value(true)
                .help("Uses the configuration from FILE"),
        )
        .arg(
            Arg::with_name("full-path")
                .long("full-path")
                .help("Returns the full path of the selected item"),
        )
        .arg(
            Arg::with_name("lame")
                .short("l")
                .long("lame")
                .help("Hides emojis"),
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .value_name("FILE")
                .takes_value(true)
                .help("Sends debugging information to FILE"),
        )
        .get_matches();

    let mut logger = FileLogger::empty();
    // If '--debug' was specified, add a file
    // so the logger actually outputs something
    if let Some(f) = matches.value_of("debug") {
        logger = logger.set_logfile(f);
        logger.log("Logging!");
    }

    // Look for boolean flags and save the state
    // in a variable for easier access
    let lame = matches.is_present("lame");
    let full_path = matches.is_present("full-path");

    // Get the string, which should be processed
    // Try to use INTPUT if defined
    // else read from the standard input
    let mut input = String::new();
    if let Some(r) = matches.value_of("INPUT") {
        input = r.to_string();
    } else {
        stdin()
            .read_to_string(&mut input)
            .expect("Failed to receive from stdin");
        // reading from stdin adds a '\n' to the end -> remove that
        input.remove(input.len() - 1);
    }

    // Open input file and read to string
    // or try the default one (~/.config/navigator/config.toml)
    let mut config = String::new();
    if let Some(c) = matches.value_of("config") {
        File::open(c)
            .expect("Failed to open config")
            .read_to_string(&mut config)
            .expect("Failed to read config");
    } else {
        let default_path = Path::new(env!("HOME")).join(".config/navigator/config.toml");
        if let Ok(mut f) = File::open(default_path) {
            f.read_to_string(&mut config)
                .expect("Failed to read from default config");
        }
    }

    // Config::read_config returns default values if the string is empty
    // and takes additional vlaues which can be configured at runtime
    // These can be also defined in the config file, but could get overwritten
    let config = ui::read_config(config.as_str(), lame);

    // Check if a seperator was provided
    // else fall back to \t (tab)
    let separator = matches.value_of("separator").unwrap_or("\t").to_string();

    // Message that get's outputted
    // Gets filled inside the for loop
    let mut message = String::new();

    // I'm too stupid to deinitalize the stdout grabber
    // that termion creates so I put that stuff into brackets
    // so it deinitializes it automatically
    {
        // Use tty instead of stdin
        // because stdin could be blocked by the user input
        let tty = termion::get_tty().expect("Could not find tty!");

        // Set up the terminal -> into raw mode
        let raw = stdout()
            .into_raw_mode()
            .expect("Failed to put the terminal into raw mode");
        let backend = TermionBackend::new(raw);
        let mut terminal = Terminal::new(backend).expect("Failed to create the terminal");

        terminal.hide_cursor().expect("Failed to hide the cursor");
        terminal.clear().expect("Failed to clear the terminal");

        let mut selected = Selectable::List;
        let mut search_widget = SearchWidget::new();
        let mut content_widget = ContentWidget::from_string(input, separator, logger);
        let mut info_widget = InfoWidget::new(content_widget.displayed.len());

        // Draw the layout for the first time
        ui::draw(
            &mut terminal,
            &content_widget,
            &search_widget,
            &info_widget,
            &selected,
            &config,
        );

        // Start listening
        for event in tty.events() {
            // If the program failed
            // to get the event, just continue
            if event.is_err() {
                continue;
            }

            match selected {
                Selectable::Search => {
                    match event.unwrap() {
                        // Must go before Key::Char(c)
                        // Switch back while keeping the search
                        //
                        // Only possible if something was found
                        // else block the switch (the user can escape with esc or search for
                        // something different)
                        Event::Key(Key::Char('\n')) => {
                            if !content_widget.displayed.is_empty() {
                                selected = Selectable::List;
                            }
                        }
                        // Add the char to the search
                        Event::Key(Key::Char(c)) => {
                            search_widget.add(c);
                            content_widget.apply_search(search_widget.get_content());
                            info_widget.update(content_widget.displayed.len());
                        }
                        // Remove the last char from the search
                        Event::Key(Key::Backspace) => {
                            search_widget.pop();
                            content_widget.apply_search(search_widget.get_content());
                            info_widget.update(content_widget.displayed.len());
                        }
                        // Switch back to the list view
                        // do not keep the search
                        Event::Key(Key::Esc) => {
                            selected = Selectable::List;
                            search_widget.clear();
                            content_widget.apply_search(search_widget.get_content());
                            info_widget.update(content_widget.displayed.len());
                        }

                        _ => {}
                    }
                }
                Selectable::List => {
                    match event.unwrap() {
                        // move up/down/left/right
                        // with the arrow or vim keys
                        Event::Key(Key::Up) | Event::Key(Key::Char('k')) => {
                            content_widget.scroll(Direction::Up);
                        }
                        Event::Key(Key::Down) | Event::Key(Key::Char('j')) => {
                            content_widget.scroll(Direction::Down);
                        }
                        // expand an element
                        // if the folder contains no element because of the search
                        // enter the folder and directly switch to the search
                        Event::Key(Key::Right) | Event::Key(Key::Char('l')) => {
                            content_widget.expand();
                            info_widget.update(content_widget.displayed.len());
                            if content_widget.displayed.is_empty() {
                                selected = Selectable::Search;
                            }
                        }
                        // go back an element
                        // if the folder contains no element because of the search
                        // enter the folder and directly switch to the search
                        Event::Key(Key::Left) | Event::Key(Key::Char('h')) => {
                            content_widget.back();
                            info_widget.update(content_widget.displayed.len());
                            if content_widget.displayed.is_empty() {
                                selected = Selectable::Search;
                            }
                        }
                        // display all elements with their whole path
                        Event::Key(Key::Char('p')) => {
                            content_widget.toggle_display_mode();
                            info_widget.update(content_widget.displayed.len());
                            if content_widget.displayed.is_empty() {
                                selected = Selectable::Search;
                            }
                        }
                        // go to the top
                        Event::Key(Key::Char('g')) => {
                            content_widget.selected = 0;
                        }
                        // go to the bottom
                        Event::Key(Key::Char('G')) => {
                            content_widget.selected = content_widget.displayed.len() - 1;
                        }
                        // switch to search widget
                        Event::Key(Key::Char('/')) => {
                            selected = Selectable::Search;
                        }
                        // print out the selected element to stdout
                        Event::Key(Key::Char('\n')) => {
                            terminal.clear().expect("Failed to clear the terminal");
                            if full_path {
                                // The slash between is not necessary because it's provided by the
                                // .get_path method
                                message.push_str(
                                    format!(
                                        "{}{}",
                                        &content_widget.get_path(),
                                        &content_widget.get_name()
                                    )
                                    .as_str(),
                                );
                            } else {
                                message.push_str(&content_widget.get_name());
                            }
                            break;
                        }
                        // Quit the program
                        Event::Key(Key::Char('q')) => {
                            terminal.clear().expect("Failed to clear the terminal");
                            break;
                        }

                        _ => {}
                    }
                }
            }

            // Update the tui
            ui::draw(
                &mut terminal,
                &content_widget,
                &search_widget,
                &info_widget,
                &selected,
                &config,
            );
        }
    }

    // Print out the selected element = message var if not empty
    // Needs to be outside the scope so the variables (particularly stdout) is dropped
    // Prints to stderr for better usability (piping etc.)
    if !message.is_empty() {
        write!(stderr(), "{}\n", message).expect("Failed to write to stderr");
    }
}
