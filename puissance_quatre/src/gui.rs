/*
use gtk::{prelude::*, self, glib, Application, ApplicationWindow, Button, Orientation, Label, ColorButton};
use std::cell::Cell;
use std::rc::Rc;
use glib::clone;
use gtk::gdk::RGBA;

const GRID_COLUMN_NB: i32 = 7;
const GRID_ROW_NB: i32 = 6;
const GREY: RGBA = RGBA::new(0.5, 0.5, 0.5, 1.0);
const RED: RGBA = RGBA::new(1.0, 0.0, 0.0, 1.0);
const YELLOW: RGBA = RGBA::new(0.5, 0.5, 0.0, 1.0);

pub fn creer_fenetre() {
    let app: Application = Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(on_activate);
    app.run();
}

fn on_activate(application: &Application) {  

    let margin = 12;
    let number = Rc::new(Cell::new(0));

    let button_increase: Button = Button::builder()
        .label("Increase to 1")
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();
    let button_decrease: Button = Button::builder()
        .label("Decrease to -1")
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();

    let label_number = Label::new(Some(&number.get().to_string()));

    button_increase.connect_clicked(clone!(@weak number, @strong button_increase, @strong button_decrease, @strong label_number => move |_| {
        number.set(number.get() + 1);
        let decrease = (&number.get() - 1).to_string();
        let nom_bouton = format!("Decrease to {}" , decrease);
        button_decrease.set_label(&nom_bouton);
        label_number.set_label(&number.get().to_string());
        let increase = (&number.get() + 1).to_string();
        let nom_bouton = format!("Increase to {}" , increase);
        button_increase.set_label(&nom_bouton);
        println!("Number = {}" , number.get());
    }));
    button_decrease.connect_clicked(clone!(@strong button_decrease, @strong button_increase, @strong label_number => move |_| {
        number.set(number.get() - 1);
        let increase = (&number.get() + 1).to_string();
        let nom_bouton = format!("Increase to {}" , increase);
        button_increase.set_label(&nom_bouton);
        label_number.set_label(&number.get().to_string());
        let decrease = (&number.get() - 1).to_string();
        let nom_bouton = format!("Decrease to {}" , decrease);
        button_decrease.set_label(&nom_bouton);
        println!("Number = {}" , number.get());
    }));
    
    /*let window_clone: Rc<ApplicationWindow> = Rc::clone(&window);
    button.connect_clicked(move |_| {
        window_clone.close();
    });*/

    let gtk_grid = gtk::Grid::builder()
        .build();
    
    for i in 0..GRID_COLUMN_NB {
        for j in 0..GRID_ROW_NB {
            let nom_bouton = format!("Bouton {} {}" , i, j);
            println!("{}", &nom_bouton);
            let button_increase: ColorButton = ColorButton::builder()
                .title(&nom_bouton)
                .rgba(&GREY)
                .build();
            gtk_grid.attach(&button_increase, i, j, 50, 100)
        }
    }

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&label_number);
    gtk_box.append(&button_decrease);
    
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .default_height(1000)
        .default_width(2000)
        .child(&gtk_box)
        .child(&gtk_grid)
        .build();

    window.present();
}
*/