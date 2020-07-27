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
pub enum Type {
    Folder(Vec<(String, Type)>), // allows unlimited expands
    Single // A single and not expandable object
}

impl Type {
    // return the content
    // of Type::Folder or panic.
    pub fn unwrap(&self) -> Vec<(String, Type)> {
        match self {
            Self::Folder(vec) => vec.to_vec(),
            Self::Single => panic!("failed to unwrap")
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
    all: Type, // represents all elements (name and expandability)
    path: Vec<String>, // specifies the path from self.all to self.current
    current: Vec<(String, Type)>, // the list the user is currently in
    pub selected: usize, // represents the currently selected element
    search: String // store the search keywords (get used in .display)
}

impl Widget for ListWidget {
    fn display(&self) -> Vec<Text> {
        let mut vec = Vec::new();
        for (name, t) in &self.current {
            // add icons for better visbility
            let elem = match t {
                Type::Folder(_) => Text::raw(format!("{}{}", "  ", name)),
                Type::Single => Text::raw(format!("   {}", name))
            };
            
            // filter out all the names
            // that do not match with self.search
            if self.search.is_empty() || name.contains(&self.search) {
                vec.push(elem);
            }
        }
        vec
    }
}

impl ListWidget {
    pub fn new(t: Type) -> Self {
        Self {
            all: t.clone(),
            path: Vec::new(),
            current: t.unwrap(),
            selected: 0,
            search: String::new()
        } 
    }

    pub fn from_string(string: String) -> Self {
        // first, try with \t
        // custom seperators are coming
        let content = Type::Folder(
            vec![
                ("One".to_string(), Type::Single),
                ("Two".to_string(), Type::Single)
            ]
        );
        Self::new(content)
    }

    // expand -> enter a folder
    pub fn expand(&mut self) {
        // check if the element is actually expandable 
        let current_element = self.current[self.selected].clone();
        if let Type::Folder(new) = current_element.1 {
            // update .current and .path
            self.current = new;
            self.path.push(current_element.0);
            // set the selected one to 0
            // to prevent index errors
            self.selected = 0;
        }
    }

    // the opposite to expand
    pub fn back(&mut self) {
        // remove the last element from path 
        self.path.pop();
        let mut new = self.all.unwrap();
        let mut match_name = |name| {
            for (n, t) in &new {
                if name == n {
                    new = t.unwrap();
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

    pub fn get(&self) -> String {
        self.current[self.selected].0.clone()
    }

    pub fn get_path(&self) -> String {
        self.path.join("/") 
    }

    pub fn apply_search(&mut self, keyword: String) {
        self.search = keyword; 
    }
}
