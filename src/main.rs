use std::process; // for exit
use std::process::Command;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, CenterBox};

use power_panel::css::load_css;
// use power_panel::icons::load_icons;

const APP_ID: &str = "me.iancleary.power-panel";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(|_| load_css());
    // app.connect_startup(|_| load_icons());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}



fn build_ui(app: &Application) {
    // Create a button_1 with label and margins
    let button_1 = Button::from_icon_name("view-refresh-symbolic");

    // Connect to "clicked" signal of `button_1`
    button_1.connect_clicked(|_| {
        Command::new("sudo")
        .arg("reboot")
        .arg("now")
        .spawn()
        .expect("reboot command failed to start");
    });

    // let button_2 = Button::from_icon_name("window-close");
    // let button_2 = Button::from_icon_name("view-conceal-symbolic");
    let button_2 = Button::from_icon_name("view-conceal-symbolic");
    // let button_2 = Button::from_icon_name("switch-off-symbolic");

    // Connect to "clicked" signal of `button_2`
    button_2.connect_clicked(|_| {
        Command::new("sudo")
        .arg("shutdown")
        .arg("now")
        .spawn()
        .expect("shutdown command failed to start");
    });

    let button_3 = Button::from_icon_name("window-close-symbolic");
    // let button_2 = Button::from_icon_name("switch-off-symbolic");

    // Connect to "clicked" signal of `button_2`
    button_3.connect_clicked(|_| {
        process::exit(1);
    });

    button_1.add_css_class("destructive-action");
    button_2.add_css_class("destructive-action");
    button_3.add_css_class("suggested-action");

    let hbox = CenterBox::new();
    hbox.set_start_widget(Some(&button_1));
    hbox.set_center_widget(Some(&button_2));
    hbox.set_end_widget(Some(&button_3));

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&hbox)
        .build();

    // Present window
    window.present();
}
