// use content::ContentCategory;
use gtk::prelude::*;
use gtk::Application;

// mod util;
// mod content;
#[allow(unused_parens)]
mod templates;
use templates::mainmenu::MainMenuWindow;

mod content;

const APP_ID : &str = "com.ambiguousname.JackboxCustomContent";

#[allow(unused_parens)]
fn main() -> Result<(), std::io::Error> {
    templates::load_resources();
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(move |app| {
        // We create the main window.
        let win = MainMenuWindow::new(app);

        // TODO: Should be a ref cell?
        content::initialize_content(win.clone());

        // Don't forget to make all widgets visible.
        win.present();
    });

    app.run();
    Ok(())
}