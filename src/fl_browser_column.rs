use fltk::{
    app::App,
    browser::{Browser, BrowserType},
    prelude::{BrowserExt, GroupExt, WidgetExt},
    window::Window,
};

const WIDTHS: &'static [i32] = &[50, 50, 50, 70, 70, 40, 40, 70, 70, 50, -1];

fn main() {
    let app = App::default();
    let mut win = Window::default()
        .with_size(900, 300)
        .with_label("Browser with columns");

    let mut browser = Browser::default().with_size(898, 298).center_of_parent();
    browser.set_column_widths(WIDTHS);
    browser.set_column_char('\t');
    browser.set_type(BrowserType::Multi);
    browser.add("USER\tPID\t%CPU\t%MEM\tVSZ\tRSS\tTTY\tSTAT\tSTART\tTIME\tCOMMAND");
    browser.add("root\t2888\t0.0\t0.0\t1352\t0\ttty3\tSW\tAug15\t0:00\t@b@f/sbin/mingetty tty3");
    browser.add("julio\t2889\t0.0\t13.0\t221352\t0\ttty3\tR\tAug15\t1:34\t@b@f/usr/local/bin/render a35 0004");
    browser.add("uucp\t2892\t0.0\t0.0\t1352\t0\tttyS0\tSW\tAug15\t0:00\t@b@f/sbin/agetty -h 19200 ttyS0 vt100");
    browser.add("root\t13115\t0.0\t0.0\t1352\t0\ttty2\tSW\tAug30\t0:00\t@b@f/sbin/mingetty tty2");
    browser.add(
        "root\t13464\t0.0\t0.0\t1352\t0\ttty1\tSW\tAug30\t0:00\t@b@f/sbin/mingetty tty1 --noclear",
    );
    win.resizable(&browser);
    win.end();
    win.show_with_env_args();

    app.run().unwrap();
}
