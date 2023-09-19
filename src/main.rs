use std::process::Command;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "me.iancleary.power";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Reboot now?")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        // button.set_label("Hello World!");

        Command::new("sudo")
        .arg("reboot")
        .arg("now")
        .spawn()
        .expect("reboot command failed to start");
    });

    let button2 = Button::builder()
        .label("shutdown now?")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button2.connect_clicked(|button2| {
        // Set the label to "Hello World!" after the button has been clicked on
        // button.set_label("Hello World!");

        Command::new("sudo")
        .arg("shutdown")
        .arg("now")
        .spawn()
        .expect("shutdown command failed to start");
    });

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.append(&button);
    hbox.append(&button2);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&hbox)
        .build();

    // Present window
    window.present();
}
