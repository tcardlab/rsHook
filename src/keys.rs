//https://www.millisecond.com/support/docs/v6/html/language/scancodes.htm
//https://www.computercraft.info/wiki/File:CC-Keyboard-Charcodes.png



pub mod keys {
  use rdev::{Key};

  pub fn scan_code(key: Key) -> String {
    let data = match key {

      Key::Alt => 56,
      Key::AltGr => 3640,
      
      Key::MetaLeft => 3675,
      Key::MetaRight => 3676,

      Key::ShiftLeft => 42,
      Key::ShiftRight => 54,

      Key::DownArrow => 57424,
      Key::LeftArrow => 57419,
      Key::RightArrow => 57421,
      Key::UpArrow => 57416,

      Key::Escape => 1,
      Key::Function => 0, // scan code doesn't exist
      Key::ControlLeft => 29,
      Key::Backspace => 14,
      Key::CapsLock => 58,
      Key::Return => 28,
      Key::Space => 57,
      Key::Tab => 15,
      
      Key::F1 => 59,
      Key::F2 => 60,
      Key::F3 => 61,
      Key::F4 => 62,
      Key::F5 => 63,
      Key::F6 => 64,
      Key::F7 => 65,
      Key::F8 => 66,
      Key::F9 => 67,
      Key::F10 => 68,
      Key::F11 => 87,
      Key::F12 => 88,
      
      Key::BackQuote => 41,

      Key::Num1 => 2,
      Key::Num2 => 3,
      Key::Num3 => 4,
      Key::Num4 => 5,
      Key::Num5 => 6,
      Key::Num6 => 7,
      Key::Num7 => 8,
      Key::Num8 => 9,
      Key::Num9 => 10,
      Key::Num0 => 11,
      Key::Minus => 12,
      Key::Equal => 13,

      Key::KeyQ => 16,
      Key::KeyW => 17,
      Key::KeyE => 18,
      Key::KeyR => 19,
      Key::KeyT => 20,
      Key::KeyY => 21,
      Key::KeyU => 22,
      Key::KeyI => 23,
      Key::KeyO => 24,
      Key::KeyP => 25,
      Key::LeftBracket => 26,
      Key::RightBracket => 27,
      Key::BackSlash => 43,

      Key::KeyA => 30,
      Key::KeyS => 31,
      Key::KeyD => 32,
      Key::KeyF => 33,
      Key::KeyG => 34,
      Key::KeyH => 35,
      Key::KeyJ => 36,
      Key::KeyK => 37,
      Key::KeyL => 38,
      Key::SemiColon => 39,
      Key::Quote => 40,

      Key::KeyZ => 44,
      Key::KeyX => 45,
      Key::KeyC => 46,
      Key::KeyV => 47,
      Key::KeyB => 48,
      Key::KeyN => 49,
      Key::KeyM => 50,
      Key::Comma => 51,
      Key::Dot => 52,
      Key::Slash => 53,

      _ => return "Error".to_string()
    };
   data.to_string()
  }
}