use gtk::{Align, ApplicationWindow, Box, Button, Entry, Orientation, prelude::*};
use gtk::{Application, glib};
use std::sync::Arc;
use tokio::runtime::Runtime;

const APP_ID: &str = "com.github.alfrendo-silalhi.yve";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    let rt = Arc::new(Runtime::new().unwrap());

    app.connect_activate(move |app| {
        build_ui(app, Arc::clone(&rt));
    });

    app.run()
}

fn build_ui(app: &Application, rt: Arc<Runtime>) {
    let url_text_box = Entry::builder()
        .placeholder_text("URL...")
        .visibility(true)
        .build();

    let button = Button::builder().label("Send").build();

    button.connect_clicked({
        let entry = url_text_box.clone();
        let rt = Arc::clone(&rt);
        move |_| {
            let url = entry.text().to_string();
            if url.is_empty() {
                return;
            }

            rt.spawn(async move {
                match reqwest::get(&url).await {
                    Ok(response) => {
                        let status = response.status();
                        match response.text().await {
                            Ok(body) => println!("Status: {}\nBody: {}", status, body),
                            Err(e) => eprintln!("Gagal baca body: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Request gagal: {}", e),
                }
            });
        }
    });

    let hbox = Box::builder()
        .orientation(Orientation::Horizontal)
        .valign(Align::Start)
        .spacing(8)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    hbox.append(&url_text_box);
    hbox.append(&button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Yve")
        .default_height(500)
        .default_width(1000)
        .child(&hbox)
        .build();

    window.present();
}
