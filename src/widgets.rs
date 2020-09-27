extern crate tui;
extern crate regex;

use regex::Regex;

use tui::widgets::ListItem;
use tui::text::{Text, Spans, Span};
use tui::style::{Style, Modifier, Color};

// represents a selection
// of all selctable widgets
pub enum Selectable {
    Search,
    List
}

// this needs to be implemented by all paragraph widgets
pub trait ParagraphWidget {
    fn get_title(&self, lame: bool, prefix: String) -> String;
    fn display(&self, lame: bool, prefix: String) -> Text;
}

// this needs to be implemented by all list widgets
pub trait ListWidget {
    fn get_title(&self, lame: bool, prefix: String) -> String;
    fn display(&self, lame: bool, prefix: String) -> Vec<ListItem>;
}

// a default entry with a name
// and an option for a subdirectory
//
// the options holds a number which refers
// to the index where it it stored
#[derive(Clone, Debug)]
pub struct Entry {
    name: String,
    next: Option<usize>
}

impl Entry {
    pub fn new(n: String, nx: Option<usize>) -> Self {
        Self {
            name: n,
            next: nx
        }
    }
    // converts and Entry to a tuple
    // reverted ::new method
    pub fn revert(&self) -> (String, Option<usize>) {
        (self.name.clone(), self.next)
    }
}

// Directions
// needed by the ContentWidget to
// represent scrolling directions
// for better readability.
pub enum Direction {
    Up,
    Down
}

pub struct SearchWidget {
    pub content: String // represents the inputted chars
}

impl ParagraphWidget for SearchWidget {
    fn get_title(&self, lame: bool, prefix: String) -> String {
        if lame {
            " Search ".to_string()
        } else {
            format!(" {} Search ", prefix)
        }
    }
    fn display(&self, _lame: bool, _prefix: String) -> Text {
        // check if the regex is valid
        // if it's not -> bold red
        if Regex::new(self.content.as_str()).is_err() {
            let spans = Spans::from(vec![Span::styled(
                self.content.clone(),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            )]);
            Text::from(spans)
        } else {
            Text::from(self.content.as_str())
        }
    }
}

impl SearchWidget {
    pub fn new() -> Self {
        Self {
            content: String::new()
        }
    }

    pub fn add(&mut self, c: char) {
        self.content.push(c); 
    }

    pub fn pop(&mut self) {
        self.content.pop();
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    pub fn clear(&mut self) {
        self.content = String::new();
    }
}

pub struct InfoWidget {
    pub count: usize // amount of elements in folder
}
impl ParagraphWidget for InfoWidget {
    fn get_title(&self, _lame: bool, _prefix: String) -> String {
        String::new()
    }
    fn display(&self, _lame: bool, _prefix: String) -> Text {
        Text::from(Span::raw(self.count.to_string()))
    }
}

impl InfoWidget {
    pub fn new(count: usize) -> Self {
        Self {
            count: count
        }
    }
    pub fn update(&mut self, new_count: usize) {
        self.count = new_count;
    }
}

pub struct ContentWidget {
    pub all: Vec<Vec<Entry>>, // represents all elements
    pub selected: usize, // represents the currently selected element
    pub displayed: Vec<Entry>, // stores the currently displayed items
    path: Vec<(String, usize)>, // specifies the path the users is currently in
    search: String // store the search keywords (get used in .display)
}

impl ListWidget for ContentWidget {
    fn get_title(&self, lame: bool, prefix: String) -> String {
        let path = self.get_path();
        if lame {
            format!(" {} ", path)
        } else {
            format!(" {} {} ", prefix, path)
        }
    }
    fn display(&self, lame: bool, prefix: String) -> Vec<ListItem> {
        let mut vec = Vec::new();
        for entry in &self.displayed {
            // add icons for better visbility
            let elem = if lame {
                ListItem::new(Text::from(entry.name.as_str()))
            } else {
                match entry.next {
                    Some(_) => ListItem::new(Text::from(Span::raw(format!("{} {}", prefix, entry.name)))),
                    None => ListItem::new(Text::from(Span::raw(format!("    {}", entry.name))))
                }
            };
            vec.push(elem);
        }

        // if the vector is empty
        // add an informative text
        if vec.is_empty() {
            if lame {
                vec.push(ListItem::new(Text::from("    Nothing found!")));
            } else {
                vec.push(ListItem::new(Text::from("❎  Nothing found!")));
            }
        }
        vec
    }
}

impl ContentWidget {
    // simply populate a basic
    // ContentWidget with default values
    pub fn new(all: Vec<Vec<Entry>>) -> Self {
        // abort if v has no entries
        if all.is_empty() {
            panic!("no content");
        }

        Self {
            all: all.clone(),
            path: vec![("".to_string(), 0)],
            selected: 0,
            displayed: all[0].clone(),
            search: String::new()
        } 
    }

    // converts the given string to a ContentWidget    
    // this is probably the holy method, that makes this project something usable
    pub fn from_string(string: String, sep: String) -> Self {
        // first, try with \t
        // custom seperators are coming
        let mut tuple_vec: Vec<Vec<Entry>> = vec![vec![]];
    
        // checks for identifiers and returns how many it found
        let find_identifiers = |mut line: String| -> usize {
            let mut count = 0;
            loop {
                if line.starts_with(&sep) {
                    count += 1; 
                    line = line.replacen(&sep, "", 1);
                    continue;
                }
                return count;
            }
        };
        
        // stores the path in indexes to the current index
        // so the code can jump back into previous folders
        let mut path = Vec::new(); 

        // stores the current index
        let mut current = 0;
        // used to compare identifiers
        let (mut count_idents_current, mut count_idents_next) = (0, 0);
    
        let mut splitted_string = string.split('\n');

        let mut current_line: String;
        let mut next_line = match splitted_string.next() {
            Some(l) => l.to_string(),
            None => panic!("String has no newlines!")
        };
    
        loop {
            // assign the already processed next_line
            // to the current_line and handle it with the
            // updated next_line
            current_line = next_line.clone();
            next_line = match splitted_string.next() {
                Some(l) => l.to_string(),
                None => {
                    tuple_vec[current].push(Entry::new(current_line, None));
                    break;
                }
            };

            // check if it starts with \t
            // and with how many \t's and removes the automatically
            count_idents_current = count_idents_next; 
            count_idents_next = find_identifiers(next_line.clone()); 
   
            next_line = next_line.replace(&sep, "");

            // entry has a new subdirectory
            match count_idents_next {
                // new subdirectory
                c if c > count_idents_current => {
                    // add a new subdirectory and save the index
                    // as Some(index) in the current vectory
                    tuple_vec.push(Vec::new());
                    let new_index = &tuple_vec.len() - 1;
                    tuple_vec[current].push(Entry::new(current_line, Some(new_index)));
               
                    // store information to find back
                    path.push(current);
                    // enter the subdirectory
                    current = new_index;
                },
                // directory gets closed
                c if c < count_idents_current => {
                    tuple_vec[current].push(Entry::new(current_line, None));
                    let difference = count_idents_current - count_idents_next;

                    // get the previous index and update the path
                    current = path[path.len() - difference];
                    for _ in 0..difference {
                        path.pop();
                    }
                },
                // in the same directory
                _ => tuple_vec[current].push(Entry::new(current_line, None))
            }
        }

        Self::new(tuple_vec)
    }
    
    // expand -> enter a folder
    pub fn expand(&mut self) {
        // check if the element is actually expandable 
        let current_element = self.displayed[self.selected].clone();
        if let Some(new) = current_element.next {
            // update .path
            self.path.push((current_element.name, new));
            // set the selected one to 0
            // to prevent index errors
            self.selected = 0;
        }
        // update the .displayed
        self.apply_search(self.search.clone());
    }

    // the opposite to expand
    pub fn back(&mut self) {
        // remove the last element from path 
        // and update .selected
        if self.path.len() != 1 {
            self.path.pop();
            self.selected = 0;
        }
        // update the .displayed
        self.apply_search(self.search.clone());
    }

    // scroll up/down
    pub fn scroll(&mut self, direction: Direction) {
        match direction {
            // scroll up, and if
            // your're already at the top, nothing happends
            Direction::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            // scroll up, and 
            // if your're already at the bottom, nothing happens
            Direction::Down => {
                if self.selected < self.displayed.len() - 1 {
                    self.selected += 1;
                }
            }
        }
    }

    pub fn get_name(&self) -> String {
        self.displayed[self.selected].name.clone()
    }

    pub fn get_current_folder(&self) -> Vec<Entry> {
        self.all[self.path[self.path.len() - 1].1].clone()
    }

    pub fn get_path(&self) -> String {
        let mut output = String::from("/");
        for (s, _) in &self.path[1..] {
            output.push_str(s);
            output.push('/');
        }
        output
    }

    // makes testing easier
    pub fn get_all_reverted(&self) -> Vec<Vec<(String, Option<usize>)>> {
        self.all.iter().map(|v| {
            v.iter().map(|e| { e.revert() })
                .collect::<Vec<(String, Option<usize>)>>()
        }).collect()
    }

    pub fn apply_search(&mut self, keyword: String) {
        self.search = keyword; 
        self.selected = 0;

        let current_folder = self.get_current_folder();
        if self.search.is_empty() {
            self.displayed = current_folder;
            return;
        }
        // filter out all the names
        // that do not match with self.search
        // if it's not empty
        let re = Regex::new(&self.search);
        // if the regex failed, do nothing
        if re.is_err() {
            return;
        }

        let re = re.unwrap();
        self.displayed = Vec::new();
        for entry in self.get_current_folder() {
            if self.search.is_empty() || re.is_match(&entry.name) {
                self.displayed.push(entry);
                // color the regex statements
                // (need some time for research to find out
                //  how to style single characters)
                // for mat in re.find_iter(&entry.name) {   
                // }
            }
        }
    }

    // checks if the current folder actually
    // contains something or just the message that nothing was found
    pub fn empty_display(&self) -> bool {
        self.displayed.is_empty()
    }
}
