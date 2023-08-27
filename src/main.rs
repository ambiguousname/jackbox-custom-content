use std::path::Path;

use gtk::gdk::Display;
// use content::ContentCategory;
use gtk::prelude::*;
use gtk::{Application, CssProvider, StyleContext};

// mod util;
// mod content;
#[allow(unused_parens)]
mod templates;
use templates::mainmenu::MainMenuWindow;

mod content;

const APP_ID : &str = "com.ambiguousname.JackboxCustomContent";

// TODO: Move somewhere more convenient?
pub const CSS : &str  = 
"
.selector .text-button {
    border:none;
    background: none;
    box-shadow: none;
}

.selector .text-button.highlight {
    background: @theme_selected_bg_color;
    color: @theme_selected_fg_color; /* Using built-in GTK theme colors. https://github.com/surajmandalcell/Gtk-Theming-Guide/blob/master/creating_gtk_themes.md */
}

";

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(CSS.as_bytes());

    StyleContext::add_provider_for_display(&Display::default().expect("Could not get display."), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}

#[allow(unused_parens)]
fn main() -> Result<(), std::io::Error> {
    println!("Start");
    templates::load_resources();
    println!("Resources loaded.");
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    println!("App built.");
    app.connect_activate(move |app| {
        println!("App activated.");
        load_css();
        println!("CSS loaded.");
        // We create the main window.
        let win = MainMenuWindow::new(app);
        println!("Window created.");

        // TODO: Should be a ref cell?
        content::initialize_content(win.clone());
        println!("Content initialized.");

        // Don't forget to make all widgets visible.
        win.present();
    });

    app.run();
    Ok(())
}