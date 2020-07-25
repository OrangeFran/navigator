use tui::widgets::Text;

// represents a selection
// of all selctable widgets
pub enum Selectable {
    Search,
    List
}

/* This needs to be implemented
 * by all widgets to ensure compaitibility
 * with Paragraphs, Lists and more creations of the
 * tui crete.
 */
trait Widget {
    fn display(&self) -> Vec<Text>;
}

pub enum Type {
    Folder(Vec<(String, Type)>), /* allows unlimited expands */
    Single /* A single and not expandable object */
}

/* Directions
 * needed by the ListWidget to
 * represent scrolling directions
 * for better readability.
 */
pub enum Direction {
    Up,
    Down
}

pub struct SearchWidget {
    content: String /* represents the inputted chars */
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
}

pub struct ListWidget {
    content: Type, /* represents all elements (name and expandability) */
    pub selected: usize /* represents the currently selected element */
}

impl Widget for ListWidget {
    fn display(&self) -> Vec<Text> {
        /* self.content.map(|fold| {
            let new_vec = match vec {
                Type::Folder(c) => c,
                Type::Single => panic!("Something failed!")
            };
            // ... 
        }).collect::<Vec<Text>>() */
        vec![Text::raw("Hehe")]
    }
}

impl ListWidget {
    pub fn new(vec: Type) -> Self {
        return Self {
            content: vec,
            selected: 0 } 
    }

    pub fn from_string(string: String) -> Self {
        let content = Type::Folder(
            vec![
                ("One".to_string(), Type::Single),
                ("Two".to_string(), Type::Single)
            ]
        );
        Self::new(content)
    }

    // scroll up/down
    pub fn scroll(&mut self, direction: Direction) {
        match direction {
            // scroll up, and if
            // your're already at the top, nothing happends
            Direction::Up => {
                // if self.selected > 0 {
                self.selected -= 1;
                // }
            }
            // scroll up, and 
            // if your're already at the bottom, nothing happens
            Direction::Down => {
                // if self.selected < self.content.len() {
                self.selected += 1;
                // }
            }
        }
    }
}
