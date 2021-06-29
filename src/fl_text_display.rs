use fltk::{
    app::{App, Scheme},
    prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt},
    text::{TextBuffer, TextDisplay},
    window::Window,
};

fn main() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let mut win = Window::default().with_size(640, 480);

    let mut text_buffer = TextBuffer::default();
    text_buffer.set_text(
        "line 0\nline 1\nline 2
line 3\nline 4\nline 5
line 6\nline 7\nline 8
line 9\nline 10\nline 11
line 12\nline 13\nline 14
line 15\nline 16\nline 17
line 18\nline 19\nline 10
line 21\nline 22\nline 23",
    );
    let mut disp = TextDisplay::new(20, 20, 640 - 40, 480 - 40, "Display");
    disp.set_buffer(text_buffer);
    win.resizable(&disp);
    win.show();

    app.run().unwrap();
}
