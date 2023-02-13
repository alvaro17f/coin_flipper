use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label, Orientation};
use rand::random;

fn main() {
    let app = Application::builder()
        .application_id("coin_flipper")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let label = Label::builder()
        .label("Pulsa el botón 'lanzar moneda' para comenzar")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button = Button::builder()
        .label("lanzar moneda")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&label);
    content.append(&button);

    let window = ApplicationWindow::builder()
        .title("Coin Flipper")
        .application(app)
        .child(&content)
        .build();

    button.connect_clicked(move |_| flip_coin(&label));

    window.show();
}

fn flip_coin(label: &Label) {
    if random() {
        label.set_text("Cara");
    } else {
        label.set_text("Cruz");
    }
}
