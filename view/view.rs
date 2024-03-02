use gtk::prelude::*;
use gtk::{Box, Button, Entry, Label};

pub struct GameView {
    pub message_label: Label,
    pub entry: Entry,
    pub button: Button,
}

impl GameView {
    pub fn new() -> Self {
        let message_label = Label::new(None);
        message_label.set_text("Adivina un número entre 1 y 100:");

        let entry = Entry::new();
        entry.set_placeholder_text(Some("Ingresa tu número aquí"));
        entry.set_activates_default(true);

        let button = Button::with_label("Adivinar");

        Self {
            message_label,
            entry,
            button,
        }
    }

    pub fn pack(&self, vbox: &Box) {
        vbox.pack_start(&self.message_label, false, false, 0);
        vbox.pack_start(&self.entry, false, false, 0);
        vbox.pack_start(&self.button, false, false, 0);
    }
}
