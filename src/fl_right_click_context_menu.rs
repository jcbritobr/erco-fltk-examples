use fltk::{
    app::{channel, event_button, event_x, event_y, App, Scheme, Sender},
    enums::{CallbackTrigger, Event},
    input::MultilineInput,
    menu::MenuItem,
    prelude::{GroupExt, InputExt, WidgetBase, WidgetExt},
    window::Window,
};

struct MyApp {
    _window: Window,
    _input: MultilineInput,
}

#[derive(Clone, Copy)]
enum Message {
    ClickDoThing1,
    ClickDoThing2,
    ClickQuit,
}

impl MyApp {
    fn new(sender: Sender<Message>) -> Self {
        let mut window = Window::default()
            .with_size(580, 460)
            .with_label("Simple popup menu");
        window.set_trigger(CallbackTrigger::Release);

        window.set_tooltip("Use right click for popup menu ...");
        window.handle(move |_, event| {
            //let event_pos = (event_x(), event_y());
            match event {
                Event::Released if event_button() == 3 => {
                    let menu = MenuItem::new(&["Do thing#1", "Do thing#2", "Quit"]);
                    if let Some(item) = menu.popup(event_x(), event_y()) {
                        match &item.label().unwrap()[..] {
                            "Do thing#1" => {
                                sender.send(Message::ClickDoThing1);
                            }
                            "Do thing#2" => {
                                sender.send(Message::ClickDoThing2);
                            }
                            "Quit" => {
                                sender.send(Message::ClickQuit);
                            }
                            _ => {}
                        }
                    }
                    true
                }
                _ => false,
            }
        });

        let mut input = MultilineInput::default()
            .with_pos(100, 200)
            .with_size(350, 50)
            .with_label("Input");
        input.set_value("Right-click anywhere on gray window area\nfor popup menu");

        window.end();
        window.show();

        let app = MyApp {
            _window: window,
            _input: input,
        };

        app
    }
}

fn main() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let (sender, receiver) = channel::<Message>();
    let _my_app = MyApp::new(sender);
    while app.wait() {
        match receiver.recv() {
            Some(Message::ClickDoThing1) => {
                println!("You choose to do a thing");
            }
            Some(Message::ClickDoThing2) => {
                println!("You choose to do a different thing");
            }
            Some(Message::ClickQuit) => {
                std::process::exit(0);
            }
            None => {}
        }
    }
}
