use gtk::IconTheme;
use gtk::gdk::Display;


pub fn load_icons() {
  // Load the icons into the icon theme
  let icon_theme = IconTheme::for_display(
      &Display::default().expect("Could not connect to a display.")
  );
  // icon_theme.set_theme_name(Some("powpow"));
  // icon_theme.add_search_path("/home/iancleary/Development/power-panel/src/icons/hicolor");

  // in dev mode, relative to the path where `cargo run` is run from
  icon_theme.add_search_path("src/icons/hicolor");

  // let mut names = icon_theme.icon_names();
  // names.sort();

  // print theme icons
  // println!("Icon names: {:?}", names);

  // let theme_name = icon_theme.theme_name();
  // println!("theme_name: {:?}", theme_name);

  // let search_path = icon_theme.search_path();
  // println!("search_path: {:?}", search_path);

  // let resource_path = icon_theme.resource_path();
  // println!("resource_path: {:?}", resource_path);
}