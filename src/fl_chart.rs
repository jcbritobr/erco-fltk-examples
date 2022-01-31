use fltk::{
    app::App,
    enums::Color,
    misc::Chart,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    window::Window,
};

fn main() {
    let app = App::default();
    let mut win = Window::default().with_size(600, 300).with_label("Chart");
    let mut chart = Chart::new(20, 20, win.width() - 40, win.height() - 40, "Simple Chart");
    win.end();
    win.resizable(&chart);
    win.show_with_env_args();

    chart.set_bounds(-125.0, 125.0);
    let mut t = 0.0;
    for _ in 0..15 {
        let val = (t as f64).sin() * 125.0;
        chart.add(val, &format!("{:.2}", val), Color::Green);
        t += 0.5;
    }

    app.run().unwrap();
}
