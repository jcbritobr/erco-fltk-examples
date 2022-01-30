use fltk::{
    app::App,
    frame::Frame,
    image::JpegImage,
    prelude::{WidgetBase, WidgetExt},
    window::Window,
};

fn main() {
    let app = App::default();
    let mut wind = Window::default()
        .with_size(800, 590)
        .with_label("Jpeg Image");

    let mut _frame = Frame::new(0, 0, 800, 600, "");
    let jpeg_image = JpegImage::load("thumbs/lotus-flower-pink.jpg").unwrap();
    _frame.set_image(Some(jpeg_image));

    wind.show_with_env_args();
    app.run().unwrap();
}
