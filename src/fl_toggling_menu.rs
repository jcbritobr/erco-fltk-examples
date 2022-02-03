use fltk::{
    app::App,
    enums::Shortcut,
    menu::{MenuBar, MenuFlag},
    prelude::{MenuExt, WidgetExt},
    window::Window,
};

fn main() {
    let app = App::default();
    let mut win = Window::default()
        .with_size(400, 300)
        .with_label("Toggling Menu at Runtime");

    let mut menubar = MenuBar::default()
        .with_pos(0, 0)
        .with_size(win.width(), 25);

    menubar.add(
        "Options/One",
        Shortcut::Ctrl | 'o',
        MenuFlag::Toggle,
        |_| {},
    );
    menubar.add(
        "Options/Two",
        Shortcut::Ctrl | 't',
        MenuFlag::Toggle,
        |_| {},
    );
    menubar.add(
        "Options/Three",
        Shortcut::Ctrl | 'r',
        MenuFlag::Toggle,
        |_| {},
    );

    win.show_with_env_args();

    if !set_menu_item_state(&mut menubar, "Options/One", false) {
        println!("Failed (1)");
    }
    
    if !set_menu_item_state(&mut menubar, "Options/Two", true) {
        println!("Failed (2)");
    }

    if !set_menu_item_state(&mut menubar, "Options/Three", true) {
        println!("Failed (3)");
    }

    app.run().unwrap();
}

fn set_menu_item_state(menu: &mut MenuBar, item: &str, state: bool) -> bool {
    if let Some(mut item) = menu.find_item(item){
        if state {
            item.set();
        } else {
            item.clear();
        }

        return true;
    }

    false
}
