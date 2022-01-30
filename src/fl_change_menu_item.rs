use fltk::{
    app::{channel, App},
    enums::Shortcut,
    menu::MenuBar,
    prelude::{GroupExt, MenuExt, WidgetExt},
    window::Window,
};

#[derive(Clone, Copy)]
pub enum Message {
    Quit,
    Change,
}

fn main() {
    let app = App::default();
    let (sender, receiver) = channel::<Message>();
    let mut win = Window::default()
        .with_size(400, 300)
        .with_label("Change menu item");

    let mut menu_bar = MenuBar::default().with_size(400, 25).with_pos(0, 0);
    menu_bar.add_emit(
        "File/Quit",
        Shortcut::Ctrl | 'q',
        fltk::menu::MenuFlag::Normal,
        sender,
        Message::Quit,
    );
    menu_bar.add_emit(
        "Edit/Change",
        Shortcut::Ctrl | 'c',
        fltk::menu::MenuFlag::Normal,
        sender,
        Message::Change,
    );
    menu_bar.add_choice("Edit/Submenu/Aaa");
    menu_bar.add_choice("Edit/Submenu/Bbb");
    win.end();
    win.show_with_env_args();
    while app.wait() {
        match receiver.recv() {
            Some(Message::Quit) => break,
            Some(Message::Change) => {
                let m = menu_bar.clone();
                let mut submenu = match m.find_item("Edit/Submenu") {
                    Some(item) => item,
                    None => continue,
                };
                submenu.set_label("New Submenu Name");

                let mut sitem = match m.find_item("Edit/New Submenu Name/Aaa") {
                    Some(item) => item,
                    None => continue,
                };
                sitem.set_label("New Aaa Name");
            }
            None => {}
        }
    }
}
