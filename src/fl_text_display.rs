use fltk::{
    app::{App, Scheme},
    prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt},
    text::{TextBuffer, TextDisplay},
    window::Window,
};

const WIDGET_PADDING: i32 = 20;
const WIDGET_WIDTH: i32 = 640;
const WIDGET_HEIGHT: i32 = 480;

fn main() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let mut win = Window::default().with_size(WIDGET_WIDTH, WIDGET_HEIGHT);

    let mut text_buffer = TextBuffer::default();
    text_buffer.set_text(
        "line 0\nline 1\nline 2\n\
            line 3\nline 4\nline 5\n\
            line 6\nline 7\nline 8\n\
            line 9\nline 10\nline 11\n\
            line 12\nline 13\nline 14\n\
            line 15\nline 16\nline 17\n\
            line 18\nline 19\nline 10\n\
            line 21\nline 22\nline 23",
    );
    let mut disp = TextDisplay::new(
        WIDGET_PADDING,
        WIDGET_PADDING,
        WIDGET_WIDTH - 40,
        WIDGET_HEIGHT - 40,
        "Display",
    );
    disp.set_buffer(text_buffer);
    win.resizable(&disp);
    win.show();

    app.run().unwrap();
}
