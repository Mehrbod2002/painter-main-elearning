use dioxus::desktop::tao;
use egui::{Color32, Key as KeyEgui};
use tao::keyboard::Key;

pub(crate) fn convert_to_buffer(color: Color32) -> [f32; 4] {
    [
        color.r().into(),
        color.g().into(),
        color.b().into(),
        color.a().into(),
    ]
}

pub(crate) fn normalized_to_rgba(normalized: [f32; 4]) -> [u8; 4] {
    let red = (normalized[0] * 255.0) as u8;
    let green = (normalized[1] * 255.0) as u8;
    let blue = (normalized[2] * 255.0) as u8;
    let alpha = (normalized[3] * 255.0) as u8;
    [red, green, blue, alpha]
}

pub(crate) fn egui_key(key: Key) -> Option<KeyEgui> {
    match key {
        Key::Character(char) => KeyEgui::from_name(char),
        Key::Enter => Some(KeyEgui::Enter),
        Key::Space => Some(KeyEgui::Space),
        Key::Backspace => Some(KeyEgui::Backspace),
        Key::Tab => Some(KeyEgui::Tab),
        _ => None,
    }
}
