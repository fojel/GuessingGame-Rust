# Guessing Game

This is a simple guessing game implemented in Rust using GTK for the GUI.

## Description

The guessing game generates a random number between 1 and 100, and the user's task is to guess that number. The user enters their guess in an entry field and clicks a button to submit it. The game provides feedback to the user based on whether the guess is too low, too high, or correct.

## Requirements

- Rust
- GTK development libraries

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/fojel/GuessingGame-Rust.git
    ```

2. Navigate to the project directory:

    ```bash
    cd GuessingGame-Rust
    ```

3. Build the project:

    ```bash
    cargo build --release
    ```

## Usage

Run the executable:

```bash
cargo run --release
