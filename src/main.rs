use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut wind = Window::default()
    .with_size(400, 400)
    .with_label("Jackbox Custom Content")
    .center_screen();

    let mut frame = Frame::new(0, 0, 400, 200, "Select an Option:");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    
    wind.make_resizable(true);
    wind.end();
    wind.show();

    but.set_callback(move |_| frame.set_label("Hello World!")); // the closure capture is mutable borrow to our button
    
    app.run().unwrap();
}