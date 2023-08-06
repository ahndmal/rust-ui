use glib::clone;
use gtk::{glib, Orientation};
use gtk::prelude::*;

const APP_ID: &str = "Rust UI App";

fn on_activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .width_request(1200)
        .height_request(800)
        .build();

    let close_button = gtk::Button::builder()
        .label("Close")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    close_button.connect_clicked(|btn| { btn.set_label("Changed") });

    let main_box = gtk::Box::builder()
        .name("buttons")
        .orientation(Orientation::Vertical)
        .tooltip_text("tooltip")
        .build();

    main_box.append(&close_button);

    let btn_box = gtk::Box::builder()
        .name("request buton")
        .orientation(Orientation::Horizontal)
        .build();

    let req_text = gtk::Text::builder().text("Method").build();
    btn_box.append(&req_text);

    let req_url = gtk::Area

    main_box.append(&btn_box);
    main_box.append(&req_url);

    window.set_child(Some(&main_box));
    window.present();
}

fn main() {
    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(on_activate);
    app.run();
}