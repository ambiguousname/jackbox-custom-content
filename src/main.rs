use gtk::{prelude::*, Application, glib::ExitCode, CssProvider, gdk::Display};
// mod util;
// mod content;
#[allow(unused_parens)]
mod templates;
use templates::mainmenu::MainMenuWindow;

mod content;
mod mod_config;
mod util;

const APP_ID : &str = "com.ambiguousname.JackboxCustomContent";

const GLOBAL_CSS : &str = "

frame.no-border {
    border: none;
    border-radius: 0;
}

";

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(GLOBAL_CSS);

    gtk::style_context_add_provider_for_display(&Display::default().expect("Could not get display."), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    println!("Loaded Global CSS.");
}

#[allow(unused_parens)]
fn main() -> ExitCode {
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
        // We create the main window.
        let win = MainMenuWindow::new(app);
        println!("Window created.");

        // TODO: Should be a ref cell?
        content::initialize_content(win.clone());
        println!("Content initialized.");

        // Don't forget to make all widgets visible.
        win.present();
    });

    app.run()
}