mod widgets;
mod render;
mod config;

use clap::{Arg, App};

use widgets::Direction;
use widgets::{Selectable, ListWidget, SearchWidget};

use std::fs::File;
use std::io::{Read, Write};
use std::io::{stdin, stdout, stderr};

use tui::terminal::Terminal;
use tui::backend::TermionBackend;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    // setup the cli app
    let matches = App::new("navigator")
        .version("0.1")
        .author("Finn H.")
        .about("Look at output with ease!")
        .arg(Arg::with_name("INPUT")
             .help("Specify input, reads from stdin if none"))
        .arg(Arg::with_name("seperator")
             .short("s")
             .long("sep")
             .takes_value(true)
             .help("Specify the seperator that the parsing is based on"))
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .takes_value(true)
             .help("Specify the path to the config file"))
        .arg(Arg::with_name("lame")
             .short("l")
             .long("lame")
             .help("Hide emojis"))
        .get_matches();

    // specifies if emojis should be hidden
    let lame = matches.is_present("lame");

    // get the string, which should be processed
    // try to use INTPUT if defined
    // else, read from the standard input
    let mut input = String::new();
    if let Some(r) = matches.value_of("INPUT") {
        input = r.to_string();
    } else {
        stdin().read_to_string(&mut input)
            .expect("Failed to receive from stdin");
    }

    // open input file and read to string
    let mut config = String::new();
    if let Some(c) = matches.value_of("config") {
        File::open(c)
            .expect("Failed to open config")
            .read_to_string(&mut config)
            .expect("Failed to read config");
    }

    // config::read_config returns default values if the string is empty
    // and takes additional vlaues which can be configured at runtime
    // these can be also defined in the config file, but could get overwritten
    let config = config::read_config(config.as_str(), lame);
    // check if a seperator was provided
    // else fall back to \t (tab)
    let seperator = matches.value_of("seperator")
        .unwrap_or("\t").to_string();

    // message that get's outputted
    // gets filled inside the for loop
    let mut message = String::new();

    // i'm too stupid to deinitalize the stdout grabber
    // that termion create so I put that stuff into brackets
    // so it deinitializes it automatically
    {    
        // use tty instead of stdin
        // because stdin could be blocked by the user input
        let tty = termion::get_tty().expect("Could not find tty!");
            
        // set up the terminal -> into raw mode
        let raw = stdout().into_raw_mode().expect("Failed to put the terminal into raw mode");
        let backend = TermionBackend::new(raw);
        let mut terminal = Terminal::new(backend).expect("Failed to create the terminal");

        terminal.hide_cursor().expect("Failed to hide the cursor");
        terminal.clear().expect("Failed to clear the terminal");

        let mut selected = Selectable::List;
        let mut search_widget = SearchWidget::new();
        let mut list_widget = ListWidget::from_string(input, seperator);

        // draw the layout for the first time
        render::draw(&mut terminal, &list_widget, &search_widget, &selected, &config);

        // start listening
        for event in tty.events() {
            // if the program failed
            // to get the event, just continue
            if event.is_err() {
                continue;
            }
        
            match selected {
                Selectable::Search => {
                    match event.unwrap() {
                        // must go befor Key::Char(c)
                        // switch back while keeping the search
                        //
                        // only possible if something was found
                        // else block the switch (the user can escape with esc or search for
                        // something different)
                        Event::Key(Key::Char('\n')) => {
                            if !list_widget.empty_display() {
                                selected = Selectable::List;
                            }    
                        }
                        // add the char to the search
                        Event::Key(Key::Char(c)) => {
                            search_widget.add(c);
                            list_widget.apply_search(search_widget.get_content());
                        }
                        // remove the last char from the search
                        Event::Key(Key::Backspace) => {
                            search_widget.pop();
                            list_widget.apply_search(search_widget.get_content());
                        }
                        // switch back to the list view
                        // do not keep the search
                        Event::Key(Key::Esc) => {
                            selected = Selectable::List;
                            search_widget.clear();
                            list_widget.apply_search(search_widget.get_content());
                        }

                        _ => {}
                    }
                }
                Selectable::List => {
                    match event.unwrap() {
                        // move up/down/left/right
                        // with the arrow or vim keys
                        Event::Key(Key::Up) | Event::Key(Key::Char('k')) => {
                            list_widget.scroll(Direction::Up);
                        }
                        Event::Key(Key::Down) | Event::Key(Key::Char('j')) => {
                            list_widget.scroll(Direction::Down);
                        }
                        // expand an element
                        // if the folder contains no element because of the search
                        // enter the folder and directly switch to the search
                        Event::Key(Key::Right) | Event::Key(Key::Char('l')) => {
                            list_widget.expand();
                            if list_widget.empty_display() {
                                selected = Selectable::Search; 
                            }
                        }
                        // go back an element
                        // if the folder contains no element because of the search
                        // enter the folder and directly switch to the search
                        Event::Key(Key::Left) | Event::Key(Key::Char('h')) => {
                            list_widget.back();
                            if list_widget.empty_display() {
                                selected = Selectable::Search; 
                            }
                        }
                        // switch to search widget
                        Event::Key(Key::Char('/')) => {
                            selected = Selectable::Search;
                        }
                        // print out the selected element to stdout
                        Event::Key(Key::Char('\n')) => {
                            terminal.clear().expect("Failed to clear the terminal");
                            message.push_str(&list_widget.get_name());
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
            }

            // update the tui
            render::draw(&mut terminal, &list_widget, &search_widget, &selected, &config);
        }
    }

    // print out the selected element = message var if not empty
    // needs to be outside the scope so the variables
    // terminal and raw get destroyed -> allows for normal output to stdout
    if !message.is_empty() {
        write!(stderr(), "{}\n", message);
    }
}


// tests that ensure that the from_string 'algorithm' works.
// Cargo test will run everytime I changed something in from_string or ListWidget
// to ensure stability.
#[cfg(test)]
mod tests {
    use super::widgets::ListWidget;
    // functions to create elements for a vector
    // make writing tests a whole less verbose
    fn single() -> (String, Option<usize>) {
        (String::from("Single"), None)
    }
    fn folder(i: usize) -> (String, Option<usize>) {
        (String::from("Folder"), Some(i))
    }

    #[test]
    fn no_folders() {
        let input = String::from("Single
Single
Single");
        let seperator = String::from("\t");
        assert_eq!(
            ListWidget::from_string(input, seperator).get_all_reverted(),
            vec![vec![single(), single(), single()]]
        );
    }

    #[test]
    fn simple_folders() {
        let input = String::from("Single\nFolder\n\tSingle\nSingle");
        let seperator = String::from("\t");
        assert_eq!(
            ListWidget::from_string(input, seperator).get_all_reverted(),
            vec![vec![single(), folder(1), single()], vec![single()]]
        );
    }

    #[test]
    fn nested_folders() {
        let input = String::from("Single\nFolder\n\tSingle\n\tFolder\n\t\tFolder\n\t\t\tSingle\n\tFolder\n\t\tSingle\nSingle");
        let seperator = String::from("\t");
        // sorry, it's a little long, hope you can read it
        assert_eq!(
            ListWidget::from_string(input, seperator).get_all_reverted(),
            vec![vec![single(), folder(1), single()], vec![single(), folder(2), folder(4)], vec![folder(3)], vec![single()], vec![single()]]
        );
    }

    #[test]
    fn nested_folders_custom_seperator() {
        let input = String::from("Single\nFolder\ntabSingle\ntabFolder\ntabtabFolder\ntabtabtabSingle\ntabFolder\ntabtabSingle\nSingle");
        let seperator = String::from("tab");
        // sorry, it's a little long, hope you can read it
        assert_eq!(
            ListWidget::from_string(input, seperator).get_all_reverted(),
            vec![vec![single(), folder(1), single()], vec![single(), folder(2), folder(4)], vec![folder(3)], vec![single()], vec![single()]]
        );
    }
}
