use fltk::prelude::WidgetType;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MenuButtonType {
    Popup1 = 1,
    Popup2,
    Popup12,
    Popup3,
    Popup13,
    Popup23,
    Popup123,
}

impl WidgetType for MenuButtonType {
    fn to_i32(self) -> i32 {
        self as i32
    }

    fn from_i32(val: i32) -> Self {
        match val {
            1 => MenuButtonType::Popup1,
            2 => MenuButtonType::Popup2,
            3 => MenuButtonType::Popup12,
            4 => MenuButtonType::Popup3,
            5 => MenuButtonType::Popup13,
            6 => MenuButtonType::Popup23,
            7 => MenuButtonType::Popup123,
            _ => MenuButtonType::Popup3,
        }
    }
}