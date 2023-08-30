use gtk::glib::ExitCode;
// use content::ContentCategory;
use gtk::prelude::*;
use gtk::Application;

// mod util;
// mod content;
#[allow(unused_parens)]
mod templates;
use templates::mainmenu::MainMenuWindow;

mod content;
mod mod_manager;
mod util;

const APP_ID : &str = "com.ambiguousname.JackboxCustomContent";

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