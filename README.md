# Number Hunt

A simple number guessing game written in Rust. Guess the hidden number with the help of "hot or cold" style hints!

## Quick Start

```bash
git clone https://github.com/VisoredPrograms/number-hunt
cd number-hunt
cargo build --release
./target/release/number-hunt
```

## How to Play

1. The game will generate a random number
2. You have a limited number of attempts to guess it
3. After each guess, you'll get a hint:
   - "You're _really_ hot!" - Very close
   - "You're warm!" - Getting closer
   - "You're cold." - Far away
   - "You're way off!" - Very far

## Configuration

Adjust these values in `main.rs` to change the game settings:

```rust
let mut attempts = 5;     // Number of attempts
let max_range = 20;   // Maximum range for random number
```

## Dependencies

- `rand` - Random number generation

## License

MIT License - see LICENSE file for details.

---

Made with Rust
