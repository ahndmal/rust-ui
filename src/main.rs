use gtk::prelude::*;

fn main() {
    let application =
        gtk::Application::new(Some("my_app"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("some app");
    window.set_default_size(350, 70);

    let button = gtk::Button::with_label("Click me!");

    window.set_child(Some(&button));

    window.show();
}