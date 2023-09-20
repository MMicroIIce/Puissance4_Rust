use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

pub fn creer_fenetre() {
    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(on_activate);
    app.run();
}

fn on_activate(application: &Application) {
    let window = ApplicationWindow::new(application);
    let button = Button::with_label("Hello World!");
    button.connect_clicked(|_| {
        window.close();
    });
    window.set_child(Some(&button));
    window.present();
}