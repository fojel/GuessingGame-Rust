mod view;
mod model;
mod controller;

use gtk::prelude::*;
use gtk::{Box, Window, WindowType};

use view::view::GameView;
use controller::controller::GameController;

fn main() {
    gtk::init().expect("Error initializing GTK.");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Adivina el número");
    window.set_default_size(300, 150);

    let vbox = Box::new(gtk::Orientation::Vertical, 5);

    let game_view = GameView::new();
    let game_controller = GameController::new(game_view);
    game_controller.start(&vbox);

    window.add(&vbox);
    window.show_all();

    gtk::main();
}
