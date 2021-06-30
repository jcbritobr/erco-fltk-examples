use fltk::{
    app::{App, Scheme},
    enums::Shortcut,
    input::MultilineInput,
    menu::{MenuButton, MenuFlag},
    prelude::{GroupExt, InputExt, MenuExt, WidgetExt},
    window::Window,
};

mod menu_button_type;
use menu_button_type::MenuButtonType;

struct MyApp {
    _window: Window,
    _menu: MenuButton,
    _input: MultilineInput,
}

fn callback(menu: &mut MenuButton) {
    let text = menu.choice().unwrap();
    match &text[..] {
        "Do thing#1" => println!("You choose to do a thing."),
        "Do thing#2" => println!("You choose to do a different thing."),
        "Quit" => std::process::exit(0),
        _ => println!("unknown"),
    }
}

impl MyApp {
    fn new() -> Self {
        let mut window = Window::default()
            .with_size(580, 460)
            .with_label("Simple popup menu");
        window.set_tooltip("Use right click for popup menu ...");

        let mut input = MultilineInput::default()
            .with_pos(100, 200)
            .with_size(350, 50)
            .with_label("Input");

        let mut menu = MenuButton::default()
            .with_size(580, 460)
            .with_pos(0, 0)
            .with_label("Popu Menu");

        menu.set_type(MenuButtonType::Popup3);
        menu.add("Do thing#1", Shortcut::Button1, MenuFlag::Normal, callback);
        menu.add("Do thing#2", Shortcut::Button2, MenuFlag::Normal, callback);
        menu.add("Quit", Shortcut::from_char('q'), MenuFlag::Normal, callback);

        input.set_value("Right-click anywhere on gray window area\nfor popup menu");
        window.end();
        window.show();

        let app = MyApp {
            _window: window,
            _menu: menu,
            _input: input,
        };

        app
    }
}

fn main() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let _my_app = MyApp::new();
    app.run().unwrap();
}
