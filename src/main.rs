// use content::ContentCategory;
use gtk::prelude::*;
use gtk::Application;

// mod util;
// mod content;
#[allow(unused_parens)]
mod templates;
use templates::mainmenu::MainMenuWindow;

#[allow(unused_parens)]
fn main() {
    templates::load_resources();

    let app = Application::builder()
        .application_id("Jackbox.Custom.Content")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = MainMenuWindow::new(app);

        // Don't forget to make all widgets visible.
        win.present();
    });

    app.run();
}