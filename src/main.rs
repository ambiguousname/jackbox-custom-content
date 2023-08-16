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

        if (win.jackbox_folder().is_none()) {
            win.toggle_content_columns_visibility(false);
            win.toggle_folder_visibility(true);
        }

        // Don't forget to make all widgets visible.
        win.present();
    });

    app.run();
}