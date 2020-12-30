This is an attempt to port [t.py][t] to Rust, and should be fully compatible with existing task files.

Made purely for educational purposes, since I'm learning Rust.

**Features**

- [ ] Task "prefixes"
- [x] List unfinished tasks (`t`)
- [ ] List finished tasks (`t --done`)
- [x] Add task to list (`t TEXT`)
- [x] Remove task from list (`t [-r|--remove] TASK`)
- [x] Edit task in list (`t [-e|--edit] TASK TEXT`)
- [ ] Finish task (`t [-f|--finish] TASK`)
- [ ] Delete list if empty (`t [-d|--delete-if-empty`)
- [ ] Search tasks (`t [-g|--grep] WORD`)

[t]: https://github.com/sjl/t
