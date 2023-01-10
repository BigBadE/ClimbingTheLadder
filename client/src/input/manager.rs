use std::collections::HashMap;
use json::JsonValue;
use json::object::Object;
use winit::event::VirtualKeyCode;
use macros::JsonResource;
use core::hashmap;

pub type PressedKey = (VirtualKeyCode, u32);
pub type KeyAction = (&'static str, fn());

//Each key is a combination of 32 bit modifier + 32 bit scancode
#[derive(JsonResource)]
pub struct InputManager {
    #[ignore_field]
    keys: HashMap<PressedKey, KeyAction>,
    #[ignore_field]
    key_map: HashMap<VirtualKeyCode, &'static str>,
}

impl InputManager {
    pub fn new() -> Self {
        return Self {
            keys: HashMap::new(),
            //6000+ characters of pain. No better way to do this
            key_map: hashmap!(VirtualKeyCode::Key1 => "1", VirtualKeyCode::Key2 => "2", VirtualKeyCode::Key3 => "3", VirtualKeyCode::Key4 => "4", VirtualKeyCode::Key5 => "5", VirtualKeyCode::Key6 => "6", VirtualKeyCode::Key7 => "7", VirtualKeyCode::Key8 => "8", VirtualKeyCode::Key9 => "9", VirtualKeyCode::Key0 => "0", VirtualKeyCode::A => "A", VirtualKeyCode::B => "B", VirtualKeyCode::C => "C", VirtualKeyCode::D => "D", VirtualKeyCode::E => "E", VirtualKeyCode::F => "F", VirtualKeyCode::G => "G", VirtualKeyCode::H => "H", VirtualKeyCode::I => "I", VirtualKeyCode::J => "J", VirtualKeyCode::K => "K", VirtualKeyCode::L => "L", VirtualKeyCode::M => "M", VirtualKeyCode::N => "N", VirtualKeyCode::O => "O", VirtualKeyCode::P => "P", VirtualKeyCode::Q => "Q", VirtualKeyCode::R => "R", VirtualKeyCode::S => "S", VirtualKeyCode::T => "T", VirtualKeyCode::U => "U", VirtualKeyCode::V => "V", VirtualKeyCode::W => "W", VirtualKeyCode::X => "X", VirtualKeyCode::Y => "Y", VirtualKeyCode::Z => "Z", VirtualKeyCode::Escape => "Escape", VirtualKeyCode::F1 => "Function 1", VirtualKeyCode::F2 => "Function 2", VirtualKeyCode::F3 => "Function 3", VirtualKeyCode::F4 => "Function 4", VirtualKeyCode::F5 => "Function 5", VirtualKeyCode::F6 => "Function 6", VirtualKeyCode::F7 => "Function 7", VirtualKeyCode::F8 => "Function 8", VirtualKeyCode::F9 => "Function 9", VirtualKeyCode::F10 => "Function 10", VirtualKeyCode::F11 => "Function 11", VirtualKeyCode::F12 => "Function 12", VirtualKeyCode::F13 => "Function 13", VirtualKeyCode::F14 => "Function 14", VirtualKeyCode::F15 => "Function 15", VirtualKeyCode::F16 => "This isn't even on my keyboard", VirtualKeyCode::F17 => "I want to know what you're even doing", VirtualKeyCode::F18 => "Function 18", VirtualKeyCode::F19 => "Function 19", VirtualKeyCode::F20 => "Literally why", VirtualKeyCode::F21 => "You don't have this key", VirtualKeyCode::F22 => "You cannot convince me", VirtualKeyCode::F23 => "That you have", VirtualKeyCode::F24 => "24 function keys", VirtualKeyCode::Snapshot => "Snapshot (??)", VirtualKeyCode::Scroll => "Scroll", VirtualKeyCode::Pause => "Pause", VirtualKeyCode::Insert => "Insert", VirtualKeyCode::Home => "Home", VirtualKeyCode::Delete => "Delete (not Backspace)", VirtualKeyCode::End => "End", VirtualKeyCode::PageDown => "Page Down", VirtualKeyCode::PageUp => "Page Up", VirtualKeyCode::Left => "Left Arrow", VirtualKeyCode::Up => "Up Arrow", VirtualKeyCode::Right => "Right Arrow", VirtualKeyCode::Down => "Down Arrow", VirtualKeyCode::Back => "Backspace", VirtualKeyCode::Return => "Enter", VirtualKeyCode::Space => "Space", VirtualKeyCode::Compose => "Compose (linux nerd)", VirtualKeyCode::Caret => "Caret", VirtualKeyCode::Numlock => "Numlock", VirtualKeyCode::Numpad0 => "Numpad 0", VirtualKeyCode::Numpad1 => "Numpad 1", VirtualKeyCode::Numpad2 => "Numpad 2", VirtualKeyCode::Numpad3 => "Numpad 3", VirtualKeyCode::Numpad4 => "Numpad 4", VirtualKeyCode::Numpad5 => "Numpad 5", VirtualKeyCode::Numpad6 => "Numpad 6", VirtualKeyCode::Numpad7 => "Numpad 7", VirtualKeyCode::Numpad8 => "Numpad 8", VirtualKeyCode::Numpad9 => "Numpad 9", VirtualKeyCode::NumpadAdd => "Numpad +", VirtualKeyCode::NumpadDivide => "Numpad /", VirtualKeyCode::NumpadDecimal => "Numpad .", VirtualKeyCode::NumpadComma => "Numpad ,", VirtualKeyCode::NumpadEnter => "Numpad Enter", VirtualKeyCode::NumpadEquals => "Numpad = (I don't have this key either)", VirtualKeyCode::NumpadMultiply => "Numpad *", VirtualKeyCode::NumpadSubtract => "Numpad -", VirtualKeyCode::AbntC1 => "Something Brazilian", VirtualKeyCode::AbntC2 => "Something also Brazilian", VirtualKeyCode::Apostrophe => "'", VirtualKeyCode::Apps => "Apps", VirtualKeyCode::Asterisk => "*", VirtualKeyCode::At => "@", VirtualKeyCode::Ax => "I don't even know", VirtualKeyCode::Backslash => "\\", VirtualKeyCode::Calculator => "Calculator?", VirtualKeyCode::Capital => "Capital?", VirtualKeyCode::Colon => ":", VirtualKeyCode::Comma => ",", VirtualKeyCode::Convert => "Convert", VirtualKeyCode::Equals => "=", VirtualKeyCode::Grave => "`", VirtualKeyCode::Kana => "Kana", VirtualKeyCode::Kanji => "Kanji", VirtualKeyCode::LAlt => "Left Alt", VirtualKeyCode::LBracket => "{", VirtualKeyCode::LControl => "Left Control", VirtualKeyCode::LShift => "Left Shift", VirtualKeyCode::LWin => "Left Windows Key (why?)", VirtualKeyCode::Mail => "Mail??", VirtualKeyCode::MediaSelect => "Media Select", VirtualKeyCode::MediaStop => "Media Stop", VirtualKeyCode::Minus => "-", VirtualKeyCode::Mute => "Mute", VirtualKeyCode::MyComputer => "My Computer", VirtualKeyCode::NavigateForward => "Next", VirtualKeyCode::NavigateBackward => "Prior", VirtualKeyCode::NextTrack => "Next Track", VirtualKeyCode::NoConvert => "No Convert", VirtualKeyCode::OEM102 => "I don't know what this key is", VirtualKeyCode::Period => ".", VirtualKeyCode::PlayPause => "Play/Pause", VirtualKeyCode::Plus => "+", VirtualKeyCode::Power => "Turn off your computer", VirtualKeyCode::PrevTrack => "Previous Track", VirtualKeyCode::RAlt => "Right Alt", VirtualKeyCode::RBracket => "}", VirtualKeyCode::RControl => "Right Control", VirtualKeyCode::RShift => "Right Shift", VirtualKeyCode::RWin => "Right Windows Key (why??)", VirtualKeyCode::Semicolon => ";", VirtualKeyCode::Slash => "/", VirtualKeyCode::Sleep => "Sleep your computer", VirtualKeyCode::Stop => "Stop", VirtualKeyCode::Sysrq => "Something weird", VirtualKeyCode::Tab => "Tab", VirtualKeyCode::Underline => "Underline?", VirtualKeyCode::Unlabeled => "Only god and you know what this key is", VirtualKeyCode::VolumeDown => "Volume down", VirtualKeyCode::VolumeUp => "Volume up", VirtualKeyCode::Wake => "Turn on your computer", VirtualKeyCode::WebBack => "Web back", VirtualKeyCode::WebFavorites => "Web favorites", VirtualKeyCode::WebForward => "Web forwards", VirtualKeyCode::WebHome => "Web home", VirtualKeyCode::WebRefresh => "Web refresh", VirtualKeyCode::WebSearch => "Web search", VirtualKeyCode::WebStop => "Turn off the internet", VirtualKeyCode::Yen => "Yen", VirtualKeyCode::Copy => "Copy", VirtualKeyCode::Paste => "Paste", VirtualKeyCode::Cut => "Cut"),
        };
    }

    pub fn add_action(&mut self, default: PressedKey, action: KeyAction) {
        self.keys.insert(default, action);
    }

    //Maps a given keycode and modifier to the function it should call
    pub fn map(&self, modifiers: u32, keycode: VirtualKeyCode) -> Option<fn()> {
        return self.keys.get(&(keycode, modifiers)).map(|tuple| tuple.1);
    }

    //Replaces the keys of all loaded entries
    pub fn load(resource: &JsonValue) -> Self {
        let mut loading = InputManager::new();
        let mut replaced = Vec::new();
        for replacing in &loading.keys {
            for (tupled, value) in resource["KEYS"].entries() {
                let mut split = tupled.split(':');
                match loading.unwrap_and_replace_or_scream(
                    split.next(), split.next(), replacing, value) {
                    Some(found) => replaced.push(found),
                    None => {}
                }
            }
        }
        for (key, replacing, value) in replaced {
            loading.keys.remove(&replacing);
            loading.keys.insert(key, value);
        }
        return __load_InputManager(loading, resource);
    }

    //Unwraps the name/modifier and replaces the given key if it matches.
    //If The modifier is invalid or missing, it screams at you.
    fn unwrap_and_replace_or_scream(&self, name: Option<&str>, modifier: Option<&str>,
                                    replacing: (&PressedKey, &KeyAction), value: &JsonValue) -> Option<(PressedKey, PressedKey, KeyAction)> {
        let key = self.key_map.get(&replacing.0.0).unwrap();
        match value {
            JsonValue::String(str) => {
                if str != key {
                    return None;
                }
            }
            _ => {
                println!("Invalid key action {}", value);
                return None;
            }
        }

        if modifier.is_none() {
            println!("No modifier! Ignoring and overwriting");
            return None;
        }

        let modifier = modifier.unwrap().parse::<u32>();
        if modifier.is_err() {
            println!("Invalid modifier for key! Ignoring and overwriting");
            return None;
        }

        let modifier = modifier.unwrap();

        //Find the VirtualKeyCode given the name. Not a good method, but it works.
        //https://stackoverflow.com/questions/59401720/how-do-i-find-the-key-for-a-value-in-a-hashmap
        let new_key = self.key_map.iter().find(|(_, value)| value == &&name.unwrap()).unwrap().0;

        return Some(((*new_key, modifier), *replacing.0, *replacing.1));
    }

    pub fn save(&self) -> JsonValue {
        let mut entries = Object::new();
        for (pressed, action) in &self.keys {
            entries.insert(format!("{}:{}", self.key_map.get(&pressed.0).unwrap(), pressed.1).as_str(),
                           JsonValue::String(action.0.to_string()))
        }
        return JsonValue::Object(entries);
    }
}