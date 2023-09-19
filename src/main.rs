use std::process::Command;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, CssProvider, IconTheme};
use gtk::gdk::Display;

const APP_ID: &str = "me.iancleary.powpow";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(|_| load_css());
    app.connect_startup(|_| load_icons());
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn load_icons() {
    // Load the icons into the icon theme
    let icon_theme = IconTheme::for_display(
        &Display::default().expect("Could not connect to a display.")
    );
    // icon_theme.set_theme_name(Some("powpow"));
    // icon_theme.add_search_path("/home/iancleary/Development/power-panel/src/icons/hicolor");

    // in dev mode, relative to the path where `cargo run` is run from
    icon_theme.add_search_path("src/icons/hicolor");

    let mut names = icon_theme.icon_names();
    names.sort();

    // print theme icons
    // println!("Icon names: {:?}", names);

    let theme_name = icon_theme.theme_name();
    println!("theme_name: {:?}", theme_name);

    // let search_path = icon_theme.search_path();
    // println!("search_path: {:?}", search_path);

    let resource_path = icon_theme.resource_path();
    println!("resource_path: {:?}", resource_path);
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
    let button_2 = Button::from_icon_name("view-conceal-symbolic");


    // Connect to "clicked" signal of `button_2`
    button_2.connect_clicked(|_| {
        Command::new("sudo")
        .arg("shutdown")
        .arg("now")
        .spawn()
        .expect("shutdown command failed to start");
    });

    button_1.add_css_class("restart");
    button_2.add_css_class("shutdown");


    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.append(&button_1);
    hbox.append(&button_2);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&hbox)
        .build();

    // Present window
    window.present();
}
