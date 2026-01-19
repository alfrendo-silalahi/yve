use gtk::{ApplicationWindow, Button, prelude::*};
use gtk::{Application, glib};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|button| {
        button.set_label("Hello world!");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK Application")
        .child(&button)
        .build();

    window.present();
}
