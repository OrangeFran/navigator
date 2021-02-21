use crate::util::FileLogger;

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use regex::Regex;

use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::ListItem;

const MAX_THREAD_AMOUNT: usize = 20;

// Represents a selection
// of all selctable widgets
pub enum Selectable {
    Search,
    List,
}

// This needs to be implemented by all paragraph widgets
pub trait ParagraphWidget {
    fn get_title(&self, lame: bool, prefix: String) -> String;
    fn display(&self, lame: bool, prefix: String) -> Text;
}

// This needs to be implemented by all list widgets
pub trait ListWidget {
    fn get_selected(&self, size: Rect) -> usize;
    fn get_title(&self, lame: bool, prefix: String) -> String;
    fn display(&self, size: Rect, lame: bool, prefix: String) -> Vec<ListItem>;
}

// A default entry with a name
// and an option for a subdirectory
//
// The options holds a number which refers
// to the index where it it stored
#[derive(Clone, Debug)]
pub struct Entry {
    pub name: String,
    pub next: Option<usize>,
    spans: Vec<Span<'static>>,
    special: Vec<(usize, Color)>,
}

impl Entry {
    pub fn new(name: String, next: Option<usize>, spans: Option<Vec<Span<'static>>>) -> Self {
        Self {
            name: name.clone(),
            // Just the default for now,
            // gets changed anyway if necessary
            spans: spans.unwrap_or(vec![Span::from(name)]),
            next: next,
            special: Vec::new(),
        }
    }
}

// Needed by the ContentWidget to
// represent scrolling directions
// for better readability.
pub enum Direction {
    Up,
    Down,
}

pub struct SearchWidget {
    // Represents the inputted chars
    pub content: String,
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
        // Check if the regex is valid
        // If it's not -> bold red
        if Regex::new(self.content.as_str()).is_err() {
            let spans = Spans::from(vec![Span::styled(
                self.content.clone(),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
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
            content: String::new(),
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
    // Amount of elements in folder
    pub count: usize,
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
        Self { count }
    }

    pub fn update(&mut self, new_count: usize) {
        self.count = new_count;
    }
}

enum DisplayMode {
    Structured,
    FullPath,
}

pub struct Content {
    // Represents all elements
    pub all: Vec<Vec<Entry>>,
    // This saves a lot of time and resources
    pub all_with_path: Vec<Entry>,
}

pub struct ContentWidget {
    pub content: Arc<Content>,
    pub displayed: Vec<Entry>,  // Stores the currently displayed items
    pub selected: usize,        // Represents the currently selected element
    path: Vec<(String, usize)>, // Usize is equal to the index of self.all
    search: String,             // Store the search keywords (get used in .display)
    mode: DisplayMode,
    logger: FileLogger,
}

impl ListWidget for ContentWidget {
    // TODO: Improve
    fn get_selected(&self, size: Rect) -> usize {
        let max_displayed = size.height as usize;
        if self.selected < max_displayed {
            self.selected
        } else {
            max_displayed
        }
    }

    fn get_title(&self, lame: bool, prefix: String) -> String {
        let path = self.get_path();
        if lame {
            format!(" /{} ", path)
        } else {
            format!(" {} /{} ", prefix, path)
        }
    }

    fn display(&self, size: Rect, lame: bool, prefix: String) -> Vec<ListItem> {
        let mut vec = Vec::new();
        let create_list_item = |entry: &Entry| -> ListItem {
            // add icons for better visbility
            let mut spans = if !lame && entry.next.is_some() {
                // add the prefix
                vec![Span::from(format!("{} ", prefix))]
            } else {
                vec![Span::from("    ")]
            };
            spans.extend(entry.spans.clone());
            ListItem::new(Text::from(Spans::from(spans)))
        };

        // Only display the entries the user can look at
        // (this saves a lot of time with bigger vectors)
        // Every line takes up a size of 3
        let max_displayed = size.height as usize;
        if self.selected < max_displayed {
            if max_displayed < self.displayed.len() {
                for entry in &self.displayed[0..max_displayed] {
                    vec.push(create_list_item(entry));
                }
            } else {
                for entry in &self.displayed[0..] {
                    vec.push(create_list_item(entry));
                }
            }
        } else {
            let end = self.selected + max_displayed;
            if end > self.displayed.len() {
                for entry in &self.displayed[(self.selected - max_displayed)..] {
                    vec.push(create_list_item(entry));
                }
            } else {
                for entry in &self.displayed[self.selected..end] {
                    vec.push(create_list_item(entry));
                }
            }
        }

        // If the vector is empty
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
    // Simply populate a basic
    // ContentWidget with default values
    pub fn new(all: Vec<Vec<Entry>>, logger: FileLogger) -> Self {
        // Abort if v has no entries
        if all.is_empty() {
            panic!("no content");
        }

        // Store the big chunks on the heap
        // because they are from now on immutable
        let temp = Arc::new(Content {
            all: all.clone(),
            all_with_path: Vec::new(),
        });

        let arc = Arc::new(Content {
            all: all,
            all_with_path: Self::get_all_displayed_path(temp),
        });

        Self {
            content: Arc::clone(&arc),
            path: vec![("".to_string(), 0)],
            displayed: Arc::clone(&arc).all[0].clone(),
            selected: 0,
            search: String::new(),
            mode: DisplayMode::Structured,
            logger: logger,
        }
    }

    // Converts the given string to a ContentWidget
    // this is probably the holy method, that makes this project something usable
    pub fn from_string(string: String, sep: String, logger: FileLogger) -> Self {
        // First, try with \t
        // Custom seperators are coming
        let mut tuple_vec: Vec<Vec<Entry>> = vec![vec![]];

        // Checks for identifiers and returns how many it found
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

        // Stores the path in indexes to the current index
        // so the code can jump back into previous folders
        let mut path = Vec::new();
        // Stores the current index
        let mut current = 0;
        // Used to compare identifiers
        let (mut count_idents_current, mut count_idents_next) = (0, 0);
        let mut splitted_string = string.split('\n');
        let mut current_line: String;
        let mut next_line = match splitted_string.next() {
            Some(l) => l.to_string(),
            None => panic!("String has no newlines!"),
        };

        loop {
            // Assign the already processed next_line
            // to the current_line and handle it with the
            // updated next_line
            current_line = next_line.clone();
            next_line = match splitted_string.next() {
                Some(l) => l.to_string(),
                None => {
                    tuple_vec[current].push(Entry::new(current_line, None, None));
                    break;
                }
            };

            // Check if it starts with \t
            // and with how many \t's and removes the automatically
            count_idents_current = count_idents_next;
            count_idents_next = find_identifiers(next_line.clone());

            next_line = next_line.replace(&sep, "");

            // Entry has a new subdirectory
            match count_idents_next {
                // New subdirectory
                c if c > count_idents_current => {
                    // Add a new subdirectory and save the index
                    // as Some(index) in the current vectory
                    tuple_vec.push(Vec::new());
                    let new_index = &tuple_vec.len() - 1;
                    tuple_vec[current].push(Entry::new(current_line, Some(new_index), None));

                    // Store information to find back
                    path.push(current);
                    // Enter the subdirectory
                    current = new_index;
                }
                // Directory gets closed
                c if c < count_idents_current => {
                    tuple_vec[current].push(Entry::new(current_line, None, None));
                    let difference = count_idents_current - count_idents_next;

                    // get the previous index and update the path
                    current = path[path.len() - difference];
                    for _ in 0..difference {
                        path.pop();
                    }
                }
                // In the same directory
                _ => tuple_vec[current].push(Entry::new(current_line, None, None)),
            }
        }

        Self::new(tuple_vec, logger)
    }

    // Expand -> enter a folder
    pub fn expand(&mut self) {
        if let DisplayMode::Structured = self.mode {
            // Check if the element is actually expandable
            let current_element = self.displayed[self.selected].clone();
            if let Some(new) = current_element.next {
                // Update .path
                self.path.push((current_element.name, new));
                // Set the selected one to 0
                // to prevent index errors
                self.selected = 0;
            }
            // Update the .displayed
            self.apply_search(self.search.clone());
        }
    }

    // The opposite to expand
    pub fn back(&mut self) {
        if let DisplayMode::Structured = self.mode {
            // Remove the last element from path
            // and update .selected
            if self.path.len() != 1 {
                self.path.pop();
                self.selected = 0;
            }
            // Update the .displayed
            self.apply_search(self.search.clone());
        }
    }

    // Scroll up/down
    pub fn scroll(&mut self, direction: Direction) {
        match direction {
            // Scroll up, and if
            // your're already at the top, nothing happends
            Direction::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            // Scroll up, and
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
        // They need to clone the actual values
        // because self.displayed discards and colors certain
        // entries, and these change need to be isolated (for .displayed only)
        match self.mode {
            DisplayMode::Structured => self.content.all[self.path[self.path.len() - 1].1].clone(),
            DisplayMode::FullPath => self.content.all_with_path.clone(),
        }
    }

    pub fn get_path(&self) -> String {
        let mut output = String::from("");
        for (s, _) in &self.path[1..] {
            output.push_str(&format!("{}/", s));
        }
        output
    }

    // Recursively go through one Entry and his children (.next elements)
    // used in conjunction with toggle_path_display_mode
    fn recursive_travel_entry(
        content: Arc<Content>,
        vec: &mut Vec<Entry>,
        mut path: String,
        mut spans: Vec<Span<'static>>,
        mut special: Vec<(usize, Color)>,
        entry: Entry,
    ) {
        // Create a new entry with no child
        path.push_str(&entry.name);
        spans.push(Span::from(entry.name.clone()));
        let mut to_add = Entry::new(path.clone(), None, Some(spans.clone()));
        to_add.special = special.clone();
        vec.push(to_add.clone());
        // Check if subelements exist
        if let Some(p) = entry.next {
            // Add a colored (red) seperator
            // Update the .displayed
            path.push('/');
            special.push((to_add.name.len(), Color::Red));
            spans.push(Span::styled("/", Style::default().fg(Color::Red)));
            // reapply the search
            // self.apply_search(self.search.clone());
            for entry in &content.all[p] {
                // Call the function again for each subelements (recursion)
                Self::recursive_travel_entry(
                    Arc::clone(&content),
                    vec,
                    path.clone(),
                    spans.clone(),
                    special.clone(),
                    entry.clone(),
                );
            }
        }
    }

    // Adds all elements with their full path as a string
    // Starts from the folder the user is currently in
    // to the selected elements -> path search
    fn get_all_displayed_path(content: Arc<Content>) -> Vec<Entry> {
        let mut vec = Vec::new();
        for entry in &content.all[0] {
            Self::recursive_travel_entry(
                Arc::clone(&content),
                &mut vec,
                String::new(),
                Vec::new(),
                Vec::new(),
                entry.clone(),
            )
        }
        vec
    }

    // Switch modes and update .displayed
    pub fn toggle_display_mode(&mut self) {
        match self.mode {
            DisplayMode::Structured => {
                self.mode = DisplayMode::FullPath;
                // You don't see this at first look but
                // apply_search actually automatically takes from
                // all_with_path if we change the mode
                self.apply_search(self.search.clone());
            }
            DisplayMode::FullPath => {
                self.mode = DisplayMode::Structured;
                self.apply_search(self.search.clone());
            }
        }
    }

    // 1. Update .search field
    // 2. Filter all the items
    // 3. Style chars that match the regex
    pub fn apply_search(&mut self, keyword: String) {
        self.search = keyword;
        let current_folder = self.get_current_folder(); // Takes around 0.2 secs
        if self.search.is_empty() {
            self.displayed = current_folder;
            return;
        }
        // If the regex failed, do nothing
        let re = match Regex::new(&self.search) {
            Ok(r) => r,
            Err(_) => return,
        };
        self.selected = 0;
        self.displayed = Vec::new(); // Takes around 0.2 secs
        let filter_and_color = |re: Regex, list: Vec<Entry>| -> Vec<Entry> {
            let mut to_send = Vec::new();
            for mut entry in list {
                // Find out if they match
                if re.is_match(&entry.name) {
                    // Color the regex statements
                    let mut index_before = 0;
                    entry.spans = Vec::new();
                    for mat in re.find_iter(&entry.name) {
                        // Add the string (not styled) up until the mathing chars
                        // All these if statement and loops check if there is a special
                        // character that should be colored differently (these are mostly
                        // '/' and seperate two entries in the DisplayMode::FullPath)
                        if index_before != mat.start() {
                            for (ind, color) in &entry.special {
                                if ind > &index_before && ind < &mat.start() {
                                    entry.spans.push(Span::from(
                                        entry.name.get(index_before..*ind).unwrap().to_string(),
                                    ));
                                    entry
                                        .spans
                                        .push(Span::styled("/", Style::default().fg(*color)));
                                    index_before = ind + 1;
                                }
                            }
                            if index_before < mat.start() {
                                entry.spans.push(Span::from(
                                    entry
                                        .name
                                        .get(index_before..mat.start())
                                        .unwrap()
                                        .to_string(),
                                ));
                            }
                            index_before = mat.start();
                        }
                        // Add the matching chars styled
                        for (ind, color) in &entry.special {
                            if ind > &index_before && ind < &mat.end() {
                                entry.spans.push(Span::styled(
                                    entry.name.get(index_before..*ind).unwrap().to_string(),
                                    Style::default().fg(Color::Blue),
                                ));
                                entry
                                    .spans
                                    .push(Span::styled("/", Style::default().fg(*color)));
                                index_before = ind + 1;
                            } else if ind == &index_before {
                                entry
                                    .spans
                                    .push(Span::styled("/", Style::default().fg(*color)));
                                index_before += 1;
                            }
                        }
                        if mat.end() > index_before {
                            entry.spans.push(Span::styled(
                                entry.name.get(index_before..mat.end()).unwrap().to_string(),
                                Style::default().fg(Color::Blue),
                            ));
                        }
                        index_before = mat.end();
                    }
                    // Add the rest of the chars (not styled)
                    for (ind, color) in &entry.special {
                        if ind > &index_before && ind < &entry.name.len() {
                            entry.spans.push(Span::from(
                                entry.name.get(index_before..*ind).unwrap().to_string(),
                            ));
                            entry
                                .spans
                                .push(Span::styled("/", Style::default().fg(*color)));
                            index_before = ind + 1;
                        } else if ind == &index_before {
                            entry
                                .spans
                                .push(Span::styled("/", Style::default().fg(*color)));
                            index_before += 1;
                        }
                    }
                    if entry.name.len() > index_before {
                        entry.spans.push(Span::from(
                            entry
                                .name
                                .get(index_before..entry.name.len())
                                .unwrap()
                                .to_string(),
                        ));
                    }
                    // Finally push it to the displayed vector
                    // which holds all entries that should get displayed to the user
                    to_send.push(entry);
                }
            }
            to_send
        };

        // Create multiple threads for each chunk
        // to speed this whole thing up
        //
        // 1. Create max MAX_THREAD_AMOUNT chunks for max MAX_THREAD_AMOUNT threads
        //    Each chunk should cotain more than MAX_THREAD_AMOUNT entries
        // 2. Create a mutix for self.displayed
        // 3. Assign each thread a chunk and run them
        // 4. Wait for the threads to finish
        let amount_of_threads = current_folder.len() / MAX_THREAD_AMOUNT;
        // Don't bother with threads if the length is under MAX_THREAD_AMOUNT
        if amount_of_threads == 0 {
            self.displayed = filter_and_color(re, current_folder);
            return;
        }
        // Reduce the amount to MAX_THREAD_AMOUNT
        // If it is bigger than MAX_THREAD_AMOUNT
        let amount_of_threads = if amount_of_threads > MAX_THREAD_AMOUNT {
            MAX_THREAD_AMOUNT
        } else {
            amount_of_threads
        };
        let amount_of_entries = current_folder.len() / amount_of_threads;
        let (tx, rx) = mpsc::channel();

        // The last thread will include the rest of the entries
        // because most of the time the amount of entries isn't a
        // multiple of 'amount_of_threads'
        for i in 0..(amount_of_threads - 1) {
            // Needs clones to be
            // moved to it's own 'process'
            let tx_clone = tx.clone();
            let re_clone = re.clone();
            let list =
                current_folder[(i * amount_of_entries)..((i + 1) * amount_of_entries)].to_vec();
            thread::spawn(move || {
                tx_clone.send(filter_and_color(re_clone, list)).unwrap();
            });
        }

        // Spawn the last thread that includes
        // the rest of the entries
        thread::spawn(move || {
            tx.send(filter_and_color(
                re,
                current_folder[((amount_of_threads - 1) * amount_of_entries)..].to_vec(),
            ))
            .unwrap();
        });

        // Wait for the threads to finish
        for _ in 0..amount_of_threads {
            self.displayed
                .append(&mut rx.recv().expect("Failed to receive from thread"));
        }
    }
}
