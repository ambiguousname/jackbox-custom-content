// use content::ContentCategory;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ResponseType, FileChooserDialog, FileChooserAction};

// mod util;
// mod content;
mod templates;
use templates::filebrowse::FileBrowseWidget;

fn main() {
    templates::load_resources();

    let app = Application::builder()
        .application_id("Jackbox.Custom.Content")
        .build();

    app.connect_activate(|app| {
        let browser = FileBrowseWidget::new();
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .title("Jackbox 7 Custom Content")
            .child(&browser)
            .build();
            
        let file_chooser = FileChooserDialog::new(Some("Select the folder for the Jackbox Party Pack 7"), Some(&win), FileChooserAction::SelectFolder, &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)]);

        file_chooser.connect("response", true, |args| {
            let response_type = ResponseType::from(args[1].get::<i32>().unwrap());
            let this = args[0].get::<FileChooserDialog>().unwrap();
            if response_type == ResponseType::Ok {
                
            }
            if response_type == ResponseType::Ok || response_type == ResponseType::Cancel {
                this.close();
            }
            None
        });

        // Don't forget to make all widgets visible.
        win.present();
    });

    app.run();
}