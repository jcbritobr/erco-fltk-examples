use fltk::{
    app::App,
    enums::{Color, Font},
    prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt},
    text::{StyleTableEntry, TextBuffer, TextDisplay},
    window::Window,
};

const WIDGET_PADDING: i32 = 20;
const WIDGET_WIDTH: i32 = 640;
const WIDGET_HEIGHT: i32 = 480;

fn main() {
    let app = App::default();
    let mut win = Window::default().with_size(WIDGET_WIDTH, WIDGET_HEIGHT);

    let mut style_entry_buffer = Vec::<StyleTableEntry>::new();
    style_entry_buffer.push(StyleTableEntry {
        color: Color::Red,
        font: Font::Courier,
        size: 16,
    });
    style_entry_buffer.push(StyleTableEntry {
        color: Color::DarkYellow,
        font: Font::Courier,
        size: 16,
    });
    style_entry_buffer.push(StyleTableEntry {
        color: Color::DarkGreen,
        font: Font::Courier,
        size: 16,
    });
    style_entry_buffer.push(StyleTableEntry {
        color: Color::Blue,
        font: Font::Courier,
        size: 16,
    });

    let mut text_buffer = TextBuffer::default();
    let mut style_buffer = TextBuffer::default();

    text_buffer.set_text(
        "Red Line 1\nYel Line 2\nGrn Line 3\nBlu Line 4\n\
            Red Line 5\nYel Line 6\nGrn Line 7\nBlu Line 8",
    );

    style_buffer.set_text("AAAAAAAAAA\nBBBBBBBBBB\nCCCCCCCCCC\nDDDDDDDDDD\nAAAAAAAAAA\nBBBBBBBBBB\nCCCCCCCCCC\nDDDDDDDDDD\n");

    let mut disp = TextDisplay::new(
        WIDGET_PADDING,
        WIDGET_PADDING,
        WIDGET_WIDTH - 40,
        WIDGET_HEIGHT - 40,
        "Display",
    );
    
    disp.set_buffer(text_buffer); // needs be called afert set_highlight_data
    disp.set_highlight_data(Some(style_buffer), style_entry_buffer);
    win.resizable(&disp);
    win.end();
    win.show_with_env_args();

    app.run().unwrap();
}
