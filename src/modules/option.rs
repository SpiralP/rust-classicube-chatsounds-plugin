use crate::modules::Module;
use classicube_sys::{Key_, Options_Get, Options_Set, OwnedString, STRING_SIZE};
use std::{ffi::CString, os::raw::c_char};

pub struct OptionModule {
  pub open_chat_key: Option<Key_>,
  pub send_chat_key: Option<Key_>,
}

impl OptionModule {
  pub fn new() -> Self {
    Self {
      open_chat_key: None,
      send_chat_key: None,
    }
  }

  pub fn get_key_from_input_name<S: AsRef<str>>(&self, s: S) -> Option<Key_> {
    let s = s.as_ref();
    INPUT_NAMES
      .iter()
      .position(|&item| item == s)
      .map(|n| n as Key_)
  }

  pub fn get<S: Into<Vec<u8>>>(&self, key: S) -> Option<String> {
    let c_key = CString::new(key).unwrap();
    let c_default = CString::new("").unwrap();

    let mut buffer: [c_char; (STRING_SIZE as usize) + 1] = [0; (STRING_SIZE as usize) + 1];
    let mut cc_string_value = classicube_sys::String {
      buffer: buffer.as_mut_ptr(),
      capacity: STRING_SIZE as u16,
      length: 0,
    };

    unsafe {
      Options_Get(c_key.as_ptr(), &mut cc_string_value, c_default.as_ptr());
    }

    let string_value = cc_string_value.to_string();

    if string_value == "" {
      None
    } else {
      Some(string_value)
    }
  }

  pub fn set<S: Into<Vec<u8>>>(&mut self, key: S, value: String) {
    let c_key = CString::new(key).unwrap();

    let cc_string_value = OwnedString::new(value);

    unsafe {
      Options_Set(c_key.as_ptr(), cc_string_value.as_cc_string());
    }
  }
}

impl Module for OptionModule {
  fn load(&mut self) {
    self.open_chat_key = self
      .get("key-Chat")
      .and_then(|s| self.get_key_from_input_name(&s));

    self.send_chat_key = self
      .get("key-SendChat")
      .and_then(|s| self.get_key_from_input_name(&s));
  }

  fn unload(&mut self) {}
}

const INPUT_NAMES: [&str; 133] = [
  "None",
  "F1",
  "F2",
  "F3",
  "F4",
  "F5",
  "F6",
  "F7",
  "F8",
  "F9",
  "F10",
  "F11",
  "F12",
  "F13",
  "F14",
  "F15",
  "F16",
  "F17",
  "F18",
  "F19",
  "F20",
  "F21",
  "F22",
  "F23",
  "F24",
  "F25",
  "F26",
  "F27",
  "F28",
  "F29",
  "F30",
  "F31",
  "F32",
  "F33",
  "F34",
  "F35",
  "ShiftLeft",
  "ShiftRight",
  "ControlLeft",
  "ControlRight",
  "AltLeft",
  "AltRight",
  "WinLeft",
  "WinRight",
  "Up",
  "Down",
  "Left",
  "Right",
  "Number0",
  "Number1",
  "Number2",
  "Number3",
  "Number4",
  "Number5",
  "Number6",
  "Number7",
  "Number8",
  "Number9",
  "Insert",
  "Delete",
  "Home",
  "End",
  "PageUp",
  "PageDown",
  "Menu",
  "A",
  "B",
  "C",
  "D",
  "E",
  "F",
  "G",
  "H",
  "I",
  "J",
  "K",
  "L",
  "M",
  "N",
  "O",
  "P",
  "Q",
  "R",
  "S",
  "T",
  "U",
  "V",
  "W",
  "X",
  "Y",
  "Z",
  "Enter",
  "Escape",
  "Space",
  "BackSpace",
  "Tab",
  "CapsLock",
  "ScrollLock",
  "PrintScreen",
  "Pause",
  "NumLock",
  "Keypad0",
  "Keypad1",
  "Keypad2",
  "Keypad3",
  "Keypad4",
  "Keypad5",
  "Keypad6",
  "Keypad7",
  "Keypad8",
  "Keypad9",
  "KeypadDivide",
  "KeypadMultiply",
  "KeypadSubtract",
  "KeypadAdd",
  "KeypadDecimal",
  "KeypadEnter",
  "Tilde",
  "Minus",
  "Plus",
  "BracketLeft",
  "BracketRight",
  "Slash",
  "Semicolon",
  "Quote",
  "Comma",
  "Period",
  "BackSlash",
  "XButton1",
  "XButton2",
  "LeftMouse",
  "RightMouse",
  "MiddleMouse",
];
