use crate::ui::Entry;

use serde_json::Value;

// Create a ContentWidget out of a string
// `sep` stands for the separator that is used to create a kind of hierarchy
// By defaullt, `/t` is used
pub fn from_separator(string: String, sep: String) -> Vec<Vec<Entry>> {
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

    tuple_vec
}

pub fn from_json(string: String) ->  Vec<Vec<Entry>> {
    // ...
    let mut tuple_vec: Vec<Vec<Entry>> = vec![vec![]];

    // Parse the json
    let json = match serde_json::from_str(&string) {
        Ok(v) = v,
        Err(e) => panic!("Failed to deserialize json"),
    };

    // Create an `Entry` for every item
    // and convert them into the correct format
    loop {
        let val: Value = json;
        match json {
            // Just add it to the vector as a string
            String(s) => s,
            Bool(v) | Number(v) => v.to_string(),
            // TODO: Display every item individually
            Array(Vec<Value>),
            // TODO: Create subdirectories, ...
            Object(Map<String, Value>),
            _ => {},
        }
    }
    
    tuple_vec
}
