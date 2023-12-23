use gtk::Window;

fn safety_window() -> Window {
    let window = Window::builder().title("TEST").build();
    window.into()
}