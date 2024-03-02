use rand::distributions::{Distribution, Uniform};
pub struct Game {
    pub(crate) secret_number: i32,
}

impl Game {
    pub fn new() -> Self {
        let secret_number = Uniform::new_inclusive(1, 100).sample(&mut rand::thread_rng());
        Self { secret_number }
    }

    pub fn check_guess(&self, guess: i32) -> GuessResult {
        if guess < 1 || guess > 100 {
            GuessResult::OutOfRange
        } else if guess < self.secret_number {
            GuessResult::TooLow
        } else if guess > self.secret_number {
            GuessResult::TooHigh
        } else {
            GuessResult::Correct
        }
    }
}

pub enum GuessResult {
    OutOfRange,
    TooLow,
    TooHigh,
    Correct,
}
