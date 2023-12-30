use gtk::{prelude::*, Application, glib::ExitCode, CssProvider, gdk::Display, AboutDialog};
// mod util;
// mod content;
#[allow(unused_parens)]
mod templates;
use templates::mainmenu::MainMenuWindow;

mod content;
mod util;

const APP_ID : &str = "com.ambiguousname.JackboxCustomContent";

// const GLOBAL_CSS : &str = "";

// fn load_css() {
//     let provider = CssProvider::new();
//     provider.load_from_string(GLOBAL_CSS);

//     gtk::style_context_add_provider_for_display(&Display::default().expect("Could not get display."), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
//     println!("Loaded Global CSS.");
// }

#[allow(unused_parens)]
fn main() -> ExitCode {
    templates::load_resources();
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_window);

    app.run()
}

fn build_window(app: &Application) {
    // load_css();

    // This works in debug for whatever reason (runtime issues, I presume.)
    #[cfg(debug_assertions)]
    {
        println!("DEBUG SLEEP");
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    let about = AboutDialog::builder()
    .application(app)
    .authors(["ambiguousname"])
    .comments("Creates mods for the Jackbox Party Pack 7.\nWith much gratitude to Jackbox Games and the developers of the Jackbox Party Pack 7.\nMade with Rust 2021, GTK 4.12 (gtk-rs 0.7.3), Serde 1.0, and open 5.0.1")
    .copyright("MIT License © 2023 ambiguousname")
    .program_name("Jackbox Custom Content")
    .version("2.0.0")
    .website("https://github.com/ambiguousname/jackbox-custom-content")
    .website_label("Source Code")
    .title("About Jackbox Custom Content")
    .license_type(gtk::License::MitX11)
    .build();
    
    // We create the main window.
    let win = MainMenuWindow::new(app, &about);
    // println!("Window created.");

    // For this to work, make a Windows10 folder in share/themes folder in the build directory. Then copy a theme there.
    // TODO: Test what needs to get added for builds.
    // default.set_gtk_theme_name(Some("Windows10"));
    // Don't forget to make all widgets visible.
    win.present();
}