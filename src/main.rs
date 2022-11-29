use glib::clone;
use gtk::glib;
use gtk::prelude::*;

const APP_ID: &str = "Rust UI App";

fn on_activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let close_button = gtk::Button::builder()
        .label("Close").height_request(30).width_request(100).name("Close").build();
    close_button.height_request();
    close_button.width_request();
    let main_stack = gtk::Stack::builder().name("Stack 1").height_request(600).build();
    let box1 = gtk::Box::builder().name("box 1").tooltip_text("tooltip").build();
    close_button.connect_clicked(clone!(@weak window => move |_| window.close()));

    window.set_child(Some(&main_stack));
    window.set_child(Some(&box1));
    window.set_child(Some(&close_button));
    window.present();
}

fn main() {
    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(on_activate);
    app.run();
}