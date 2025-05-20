# Lottery Ticket Generator

A simple GUI for generating lottery tickets. The purpose of this application is test desktop development in RUST.

## Roadmap
- [X] Convert current work to Rust
- [ ] Add database for ticket and preset data
- [ ] Create database interface in Rust
- [ ] Implement the following windows
    - [ ] Create/Update ticket preset
    - [ ] View current ticket presets
    - [ ] Generate ticket
    - [ ] View previously generated tickets

## Quick Start
### Setup
- [GTK Dependencies](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html)

### Deployment 
Buidl and run code
```
cargo run
```

Create a release version of the code to run
```
cargo build --release
```

### Testing
#### Fake DB Interface
A quick and dirty way to run all the tests sequentially is to use the following command.
```bash 
cargo test -- --test-threads 1
```