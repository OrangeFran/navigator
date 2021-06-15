# Things to do in the (near) feature

### Small fixes

- [x] Do not allow scrolling over files you don't see (appears when searching for files).

### Currently working on

- [ ] Improve speed with larger inputs
    - [x] Move large data vectors to the heap and work with pointers (Arc)
    - [ ] Let the tui only display the data you see on the screen

### Plans

- [ ] Improve error handling (maybe use the `anyhow` create?)

- [x] Add live search. It updates while searching so you find your stuff better.
- [x] Allow searching with regex.

- [ ] Idea: work with json and toml

- [ ] Display some information about the stuff in the current folder, like size etc.
    - [x] Added amount of entries.
- [ ] More configuration options.
    - [x] Choose which emojis you want to use.
    - [ ] Choose color of regex highlights.
