// This only compiles
// if "cargo test" was run
#[cfg(test)]
mod test {
    use crate::util::FileLogger;
    use crate::ui::ContentWidget;
    use crate::ui::Entry;

    // Tests that ensure that the from_string 'algorithm' works.
    // "cargo test" will run everytime I changed something in from_string or ContentWidget
    // to ensure stability.
    impl Entry {
        // Converts an Entry to a tuple
        // reverted ::new method
        pub fn revert(&self) -> (String, Option<usize>) {
            (self.name.clone(), self.next)
        }
    }

    impl ContentWidget {
        // Makes testing easier
        pub fn get_all_reverted(&self) -> Vec<Vec<(String, Option<usize>)>> {
            self.content.all
                .iter()
                .map(|v| {
                    v.iter()
                        .map(|e| e.revert())
                        .collect::<Vec<(String, Option<usize>)>>()
                })
                .collect()
        }
    }

    // Functions to create elements for a vector
    // Make writing tests a whole less verbose
    fn single() -> (String, Option<usize>) {
        (String::from("Single"), None)
    }
    fn folder(i: usize) -> (String, Option<usize>) {
        (String::from("Folder"), Some(i))
    }

    #[test]
    fn no_folders() {
        let logger = FileLogger::empty();
        let input = String::from("Single\nSingle\nSingle");
        let seperator = String::from("\t");
        assert_eq!(
            ContentWidget::from_string(input, seperator, logger).get_all_reverted(),
            vec![vec![single(), single(), single()]]
        );
    }

    #[test]
    fn simple_folders() {
        let logger = FileLogger::empty();
        let input = String::from("Single\nFolder\n\tSingle\nSingle");
        let seperator = String::from("\t");
        assert_eq!(
            ContentWidget::from_string(input, seperator, logger).get_all_reverted(),
            vec![vec![single(), folder(1), single()], vec![single()]]
        );
    }

    #[test]
    fn nested_folders() {
        let logger = FileLogger::empty();
        let input = String::from("Single\nFolder\n\tSingle\n\tFolder\n\t\tFolder\n\t\t\tSingle\n\tFolder\n\t\tSingle\nSingle");
        let seperator = String::from("\t");
        // Sorry, it's a little long, hope you can read it
        assert_eq!(
            ContentWidget::from_string(input, seperator, logger).get_all_reverted(),
            vec![
                vec![single(), folder(1), single()],
                vec![single(), folder(2), folder(4)],
                vec![folder(3)],
                vec![single()],
                vec![single()]
            ]
        );
    }

    #[test]
    fn nested_folders_custom_seperator() {
        let logger = FileLogger::empty();
        let input = String::from("Single\nFolder\ntabSingle\ntabFolder\ntabtabFolder\ntabtabtabSingle\ntabFolder\ntabtabSingle\nSingle");
        let seperator = String::from("tab");
        // Sorry, it's a little long, hope you can read it
        assert_eq!(
            ContentWidget::from_string(input, seperator, logger).get_all_reverted(),
            vec![
                vec![single(), folder(1), single()],
                vec![single(), folder(2), folder(4)],
                vec![folder(3)],
                vec![single()],
                vec![single()]
            ]
        );
    }
}
