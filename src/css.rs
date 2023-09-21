use gtk::CssProvider;
use gtk::gdk::Display;

pub fn load_css() {
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
