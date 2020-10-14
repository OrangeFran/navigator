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
    pub name: String,
    spans: Vec<Span<'static>>,
    pub next: Option<usize>
}

impl Entry {
    pub fn new(n: String, nx: Option<usize>) -> Self {
        Self {
            name: n.clone(),
            // just the default for now, 
            // gets changed anyway if necessary
            spans: vec![Span::from(n)],
            next: nx
        }
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
        Text::from(Span::raw(format!("{} ", self.count)))
    }
}

impl InfoWidget {
    pub fn new(count: usize) -> Self {
        Self {
            count
        }
    }
    pub fn update(&mut self, new_count: usize) {
        self.count = new_count;
    }
}

enum DisplayMode {
    Structured,
    FullPath
}

pub struct ContentWidget {
    pub all: Vec<Vec<Entry>>, // represents all elements
    pub selected: usize, // represents the currently selected element
    pub displayed: Vec<Entry>, // stores the currently displayed items
    path: Vec<(String, usize)>, // specifies the path the users is currently in (usize is equal to the index of self.all)
    search: String, // store the search keywords (get used in .display)
    mode: DisplayMode
}

impl ListWidget for ContentWidget {
    fn get_title(&self, lame: bool, prefix: String) -> String {
        let path = self.get_path();
        if lame {
            format!(" /{} ", path)
        } else {
            format!(" {} /{} ", prefix, path)
        }
    }
    fn display(&self, lame: bool, prefix: String) -> Vec<ListItem> {
        let mut vec = Vec::new();
        for entry in &self.displayed {
            // add icons for better visbility
            let mut spans = if !lame && entry.next.is_some() {
                // add the prefix
                vec![Span::from(format!("{} ", prefix))]
            } else {
                vec![Span::from("    ")]
            };
            spans.extend(entry.spans.clone());
            vec.push(ListItem::new(Text::from(Spans::from(spans))));
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
            search: String::new(),
            mode: DisplayMode::Structured
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
        if let DisplayMode::Structured = self.mode {
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
    }

    // the opposite to expand
    pub fn back(&mut self) {
        if let DisplayMode::Structured = self.mode {
            // remove the last element from path 
            // and update .selected
            if self.path.len() != 1 {
                self.path.pop();
                self.selected = 0;
            }
            // update the .displayed
            self.apply_search(self.search.clone());
        }
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

    fn get_current_folder(&mut self) -> Vec<Entry> {
        match self.mode {
            DisplayMode::Structured => self.all[self.path[self.path.len() - 1].1].clone(),
            DisplayMode::FullPath => self.get_all_displayed_path()
        }
    }

    pub fn get_path(&self) -> String {
        let mut output = String::from("");
        for (s, _) in &self.path[1..] {
            output.push_str(&format!("{}/", s));
        }
        output
    }

    // recursively go through one Entry and his children (.next elements)
    // used in conjunction with toggle_path_display_mode
    fn recursive_travel_entry(&mut self, mut path: String, mut spans: Vec<Span<'static>>, entry: Entry, vec: &mut Vec<Entry>) {
        // create a new entry with no child
        let mut to_add = Entry::new(format!("{}{}", path, entry.name), None);
        to_add.spans = spans.clone();
        to_add.spans.push(Span::from(entry.name.clone()));
        vec.push(to_add);
        // add subelements if they exist
        if let Some(p) = entry.next {
            path.push_str(&format!("{}/", entry.name));
            spans.push(Span::from(entry.name.clone()));
            // and add a colored (red) seperator
            // update the .displayed
            // reapply the search
            self.apply_search(self.search.clone());
            spans.push(Span::styled("/", Style::default().fg(Color::Red)));
            for entry in self.all[p].clone() {
                // call the function again (recursion)
                self.recursive_travel_entry(path.clone(), spans.clone(), entry, vec);
            }
        }
    }

    // adds all elements with their full path as a string
    // starts from the folder the user is currently in
    // to the selected elements -> path search
    fn get_all_displayed_path(&mut self) -> Vec<Entry> {
        let mut vec = Vec::new();
        for entry in self.all[self.path[self.path.len() - 1].1].clone() {
            self.recursive_travel_entry(String::new(), Vec::new(), entry, &mut vec);
        }
        vec
    }

    // switch modes and update .displayed
    pub fn toggle_display_mode(&mut self) {
        match self.mode {
            DisplayMode::Structured => {
                self.mode = DisplayMode::FullPath;
                self.displayed = self.get_all_displayed_path();
                self.apply_search(self.search.clone());
            },
            DisplayMode::FullPath => {
                self.mode = DisplayMode::Structured;
                self.displayed = self.all[self.path[self.path.len() - 1].1].clone();
                self.apply_search(self.search.clone());
            }
        }
    }

    // - update .search field
    // - filter all the items
    // - style chars that match the regex
    pub fn apply_search(&mut self, keyword: String) {
        self.search = keyword; 
        let current_folder = self.get_current_folder();
        if self.search.is_empty() {
            self.displayed = current_folder;
            return;
        }
        // if the regex failed, do nothing
        let re = match Regex::new(&self.search) {
            Ok(r) => r,
            Err(_) => return
        };
        // for safety reasons select the first element
        self.selected = 0;
        self.displayed = Vec::new();
        for mut entry in current_folder {
            // find out if they match
            if self.search.is_empty() || re.is_match(&entry.name) {
                // color the regex statements
                entry.spans = Vec::new();
                let mut index_before = 0;
                for mat in re.find_iter(&entry.name) {   
                    // add the string (not styled) up until the mathing chars
                    if index_before != mat.start() {
                        entry.spans.push(Span::from(
                            entry.name.get(index_before..mat.start()).unwrap().to_string()
                        ));
                    }
                    // add the matching chars styled
                    entry.spans.push(Span::styled(
                        entry.name.get(mat.start()..mat.end()).unwrap().to_string(), 
                        Style::default().fg(Color::Blue)
                    ));
                    index_before = mat.end();
                }
                // add the rest of the chars (not styled)
                entry.spans.push(Span::from(
                    entry.name.get(index_before..entry.name.len()).unwrap().to_string()
                ));
                // finally push it to the displayed vector
                // which holds all entries that should get displayed to the user
                self.displayed.push(entry);
            }
        }
    }
}
