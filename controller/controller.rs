use crate::model::model::{Game, GuessResult};
use crate::view::view::GameView;
use gtk::prelude::*;

pub struct GameController {
    game: Game,
    view: GameView,
}

impl GameController {
    pub fn new(view: GameView) -> Self {
        let game = Game::new();
        Self { game, view }
    }

    pub fn start(&self, vbox: &gtk::Box) {
        self.view.pack(vbox);

        let cloned_view = self.view.clone();
        let secret_number = self.game.secret_number;

        self.view.button.connect_clicked(move |_| {
            let guess: i32 = match cloned_view.entry.get_text().as_str().parse() {
                Ok(num) => num,
                Err(_) => {
                    cloned_view
                        .message_label
                        .set_text("¡Ingresa un número válido!");
                    return;
                }
            };

            match guess {
                1..=100 => {
                    let result = self.game.check_guess(guess);
                    match result {
                        GuessResult::OutOfRange => {
                            cloned_view
                                .message_label
                                .set_text("El número debe estar entre 1 y 100.");
                        }
                        GuessResult::TooLow => {
                            cloned_view
                                .message_label
                                .set_text("¡Demasiado bajo! Inténtalo de nuevo.");
                        }
                        GuessResult::TooHigh => {
                            cloned_view
                                .message_label
                                .set_text("¡Demasiado alto! Inténtalo de nuevo.");
                        }
                        GuessResult::Correct => {
                            cloned_view
                                .message_label
                                .set_text("¡Correcto! Has adivinado el número.");
                        }
                    }
                }
                _ => {
                    cloned_view
                        .message_label
                        .set_text("El número debe estar entre 1 y 100.");
                }
            }
        });
    }
}
