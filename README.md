# spaceshooter-rust
A little game written in Rust and running on ncurses.

# Installation
To compile and run the project, run:
```
cargo build --release
```

If you want some debug info, build the game like this:
```
cargo build --features debug
```

After building, run the program with:
```
cargo run
```

# Notes
* The `ascii/` folder contains drafts and works-in-progress for my ASCII art, but its not accessed by the program. The actual, in-game ASCII art is hard-coded. 
