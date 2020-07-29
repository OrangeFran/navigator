use tui::widgets::Text;

// represents a selection
// of all selctable widgets
pub enum Selectable {
    Search,
    List
}

// This needs to be implemented
// by all widgets to ensure compaitibility
// with Paragraphs, Lists and more creations of the
// tui crate.
pub trait Widget {
    fn display(&self) -> Vec<Text>;
}

#[derive(Clone)]
pub struct Entry {
    pub name: String, // gets displayed
    pub next: Option<Vec<Entry>> // the subdirectory
}

impl Entry {
    // follow the .next to the next entry
    // and return it / panic
    pub fn follow(&self) -> Vec<Self> {
        match &self.next {
            Some(subdir) => subdir.to_vec(),
            None => panic!("could not call .follow(): no subdirectory") 
        }
    }
}

// Directions
// needed by the ListWidget to
// represent scrolling directions
// for better readability.
pub enum Direction {
    Up,
    Down
}

pub struct SearchWidget {
    pub content: String // represents the inputted chars
}

impl Widget for SearchWidget {
    fn display(&self) -> Vec<Text> {
        vec![Text::raw(self.content.clone())]
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

pub struct ListWidget {
    all: Entry, // represents all elements
    path: Vec<String>, // specifies the path from self.all to self.current
    current: Vec<Entry>, // the list the user is currently in
    pub selected: usize, // represents the currently selected element
    search: String // store the search keywords (get used in .display)
}

impl Widget for ListWidget {
    fn display(&self) -> Vec<Text> {
        let mut vec = Vec::new();
        for entry in &self.current {
            // add icons for better visbility
            let elem = match entry.next {
                Some(_) => Text::raw(format!("{}{}", "  ", entry.name)),
                None => Text::raw(format!("   {}", entry.name))
            };
            
            // filter out all the names
            // that do not match with self.search
            if self.search.is_empty() || entry.name.contains(&self.search) {
                vec.push(elem);
            }
        }
        vec
    }
}

impl ListWidget {
    pub fn new(v: Vec<Entry>) -> Self {
        // abort if v has no entries
        if v.is_empty() {
            panic!("no content");
        }

        // create the root and connect the entries
        let all = Entry {
            name: "/".to_string(),
            next: Some(v.clone())
        };

        Self {
            all: all,
            path: Vec::new(),
            current: v,
            selected: 0,
            search: String::new()
        } 
    }

    pub fn from_string(string: String) -> Self {
        // first, try with \t
        // custom seperators are coming
        let mut vec: Vec<Vec<(String, Option<usize>)>> = vec![Vec::new()];
        let mut current, previous, next, into = 0;
    
        // checks for identifiers and returns 
        // how many it found
        let find_identifiers = |s| -> usize {
            let count = 0;
            let all_chars = line.chars();
            loop {
                if let Some('\t') = all_chars.next() {
                    count += 1;
                    continue;
                }
                break count;
            } 
        };

        let count_idents_previous, count_idents_current, count_idents_next = 0;

        for line in string.split('\n') {
            // check if it starts with \t
            // and with how many \t's
            let count_idents_previous = count_idents_current; 
            let count_idents_current = count_idents_next; 
            let count_idents_next = find_identifiers(line); 

            if count_idents_current == into {
                // find out if the Entry has a subdirectory or not
                if count_idents_next > count_idents_current {
                    // add a new subdirectory and save the index
                    // as Some(index) in the current vectory
                    vec.push(Vec::new());
                    vec[current].push((line, Some(vec.len() - 1)));
                // ...
                } else if count_idents_next < count_idents_current {
                    // vec[
                }
            }
        }

        Self::new(vec)
    }

    // expand -> enter a folder
    pub fn expand(&mut self) {
        // check if the element is actually expandable 
        let current_element = self.current[self.selected].clone();
        if let Some(new) = current_element.next {
            // update .current and .path
            self.current = new;
            self.path.push(current_element.name);
            // set the selected one to 0
            // to prevent index errors
            self.selected = 0;
        }
    }

    // the opposite to expand
    pub fn back(&mut self) {
        // remove the last element from path 
        self.path.pop();
        let mut new = self.all.follow();
        let mut match_name = |name| {
            for n in &new {
                if name == &n.name {
                    new = n.follow();
                    return;
                }
            }
        };
        for name in &self.path {
            match_name(name);
        }
        // update .current and .selected
        self.current = new;
        self.selected = 0;
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
                if self.selected < self.current.len() - 1 {
                    self.selected += 1;
                }
            }
        }
    }

    pub fn get_name(&self) -> String {
        self.current[self.selected].name.clone()
    }

    pub fn get_path(&self) -> String {
        format!("/{}", self.path.join("/"))
    }

    pub fn apply_search(&mut self, keyword: String) {
        self.search = keyword; 
    }
}
