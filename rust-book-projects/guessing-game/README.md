# 🤔 rusty_guess

A simple number guessing game built in **Rust**, inspired by [The Rust Programming Language Book](https://doc.rust-lang.org/book/).  
This project was created to get hands-on experience with Rust’s core concepts like ownership, pattern matching, input/output, and error handling.

## 🎯 Overview

The game picks a random number between **1 and 100**, and the player has to guess it.  
After each guess, the program tells you whether your guess is **too low**, **too high**, or **correct**.

It’s a small project, but it helped me understand:
- Using external crates (`rand`)
- Handling user input
- Comparing and matching values
- Loops and program flow in Rust
- Basic error handling and type conversions

## ⚙️ Installation

Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

Clone this repository:

```bash
git clone https://github.com/devwraithe/rusty_guess.git
cd rusty_guess
```

Build the project:

```bash
cargo build
```

Or build and run in one go:

```bash
cargo run
```

## 🧩 How to Play

1. Run the program
```bash
cargo run
```

2. Enter the guess when prompted. You can guess between 1 and 100.
3. The program will tell you if the guess is too high, too low or correct.
4. Keep guessing until you win.

Example:

```bash
Guess the number!
Please input your guess:
> 50
Too small!
> 75
Too big!
> 63
You win!
```

## 📚 What I Learned

* How to use the rand crate for random number generation.
* Working with mutable variables and shadowing.
* Using loop, match, and Result for clean control flow.
* Borrowing rules and ownership in simple I/O contexts.

## 💬 Acknowledgements

Followed the tutorial frommm The Rust Book to build this project. Thanks to the Rust community and the authors of The Rust Programming Language Book for making Rust fun to learn!
