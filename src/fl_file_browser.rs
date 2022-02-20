use fltk::{app::App, browser::{FileBrowser, BrowserType}, prelude::{WidgetExt, BrowserExt, GroupExt}, window::Window};

fn main() {
    let app = App::default();
    let mut win = Window::default()
        .with_size(300, 400)
        .with_label("Fl_file_browser");

    let mut browser = FileBrowser::default()
        .with_size(280, 380)
        .center_of_parent();
    
    browser.load(".").unwrap();
    browser.set_type(BrowserType::Hold);
    win.end();

    win.resizable(&browser);

    win.show_with_env_args();
    app.run().unwrap();
}
