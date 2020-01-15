# Tetris

Practice [Rust](https://www.rust-lang.org/) by writing a [tetris](https://en.wikipedia.org/wiki/Tetris_(Game_Boy_video_game)) game.


## Dev

```
nix-shell -p rustup sdl2

# play
[nix-shell]$ cargo build && ./target/debug/tetris

# testing
[nix-shell]$ cargo test

# coding style
[nix-shell]$ cargo clippy
```
