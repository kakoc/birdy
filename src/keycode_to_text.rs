use winit::event::{ElementState, KeyboardInput, ScanCode, VirtualKeyCode};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    pub global_chars_offset_from_beginning: usize,
    pub global_lines_offset_from_beginning: usize,
    pub px_coordinates: (usize, usize),
}

pub static KEYCODES_WHICH_MODIFY: &[VirtualKeyCode] = &[
    VirtualKeyCode::A,
    VirtualKeyCode::B,
    VirtualKeyCode::C,
    VirtualKeyCode::D,
    VirtualKeyCode::E,
    VirtualKeyCode::F,
    VirtualKeyCode::G,
    VirtualKeyCode::H,
    VirtualKeyCode::I,
    VirtualKeyCode::J,
    VirtualKeyCode::K,
    VirtualKeyCode::L,
    VirtualKeyCode::M,
    VirtualKeyCode::N,
    VirtualKeyCode::O,
    VirtualKeyCode::P,
    VirtualKeyCode::Q,
    VirtualKeyCode::R,
    VirtualKeyCode::S,
    VirtualKeyCode::T,
    VirtualKeyCode::U,
    VirtualKeyCode::V,
    VirtualKeyCode::W,
    VirtualKeyCode::X,
    VirtualKeyCode::Y,
    VirtualKeyCode::Z,
    //
    VirtualKeyCode::Key1,
    VirtualKeyCode::Key2,
    VirtualKeyCode::Key3,
    VirtualKeyCode::Key4,
    VirtualKeyCode::Key5,
    VirtualKeyCode::Key6,
    VirtualKeyCode::Key7,
    VirtualKeyCode::Key8,
    VirtualKeyCode::Key9,
    VirtualKeyCode::Key0,
    //
    VirtualKeyCode::Back,
    VirtualKeyCode::Space,
    VirtualKeyCode::Return,
    VirtualKeyCode::Tab,
    //
    VirtualKeyCode::Escape,
];

#[rustfmt::skip]
pub static SCANCODES_WHICH_MODIFY: &[ScanCode] = &[
 // ~   %  `
    41, 2, 13,
 // $   &  [  {  }  (  =  *  )  +   ]   !   #
    41, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,

 // ;   ,   .   /   @   \   -   '
    16, 17, 18, 26, 27, 43, 40, 44,
 // :   <   >   ?   ^   |   _   "
    16, 17, 18, 26, 27, 43, 40, 44


];

pub fn handle_keycode_key_press(content: &mut String, event: KeyboardInput, cursor: &mut Cursor) {
    #[allow(deprecated)]
    if let KeyboardInput {
        virtual_keycode: Some(code),
        state: ElementState::Pressed,
        modifiers,
        ..
    } = event
    {
        let (l, r) = content.split_at(cursor.global_chars_offset_from_beginning);
        let (modified, new_cursor_position) = match code {
            VirtualKeyCode::A => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "a".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "a"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::B => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "b".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "b"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::C => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "c".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "c"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::D => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "d".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "d"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::E => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "e".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "e"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::F => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "f".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "f"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::G => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "g".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "g"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::H => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "h".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "h"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::I => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "i".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "i"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::J => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "j".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "j"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::K => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "k".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "k"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::L => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "l".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "l"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::M => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "m".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "m"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::N => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "n".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "n"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::O => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "o".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "o"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::P => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "p".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "p"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::Q => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "q".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "q"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::R => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "r".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "r"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::S => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "s".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "s"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::T => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "t".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "t"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::U => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "u".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "u"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::V => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "v".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "v"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::W => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "w".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "w"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::X => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "x".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "x"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::Y => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "y".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "y"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            VirtualKeyCode::Z => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "z".to_uppercase()),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "z"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            _ if code == VirtualKeyCode::Key1 && modifiers.shift() => (
                format!("{l}{}{r}", "1"),
                cursor.global_chars_offset_from_beginning + 1,
            ),
            _ if code == VirtualKeyCode::Key2 && modifiers.shift() => (
                format!("{l}{}{r}", "2"),
                cursor.global_chars_offset_from_beginning + 1,
            ),
            _ if code == VirtualKeyCode::Key3 && modifiers.shift() => (
                format!("{l}{}{r}", "3"),
                cursor.global_chars_offset_from_beginning + 1,
            ),
            _ if code == VirtualKeyCode::Key4 && modifiers.shift() => (
                format!("{l}{}{r}", "4"),
                cursor.global_chars_offset_from_beginning + 1,
            ),
            _ if code == VirtualKeyCode::Key5 && modifiers.shift() => (
                format!("{l}{}{r}", "5"),
                cursor.global_chars_offset_from_beginning + 1,
            ),
            _ if code == VirtualKeyCode::Key6 && modifiers.shift() => (
                format!("{l}{}{r}", "6"),
                cursor.global_chars_offset_from_beginning + 1,
            ),
            _ if code == VirtualKeyCode::Key7 && modifiers.shift() => (
                format!("{l}{}{r}", "7"),
                cursor.global_chars_offset_from_beginning + 1,
            ),
            _ if code == VirtualKeyCode::Key8 && modifiers.shift() => (
                format!("{l}{}{r}", "8"),
                cursor.global_chars_offset_from_beginning + 1,
            ),
            _ if code == VirtualKeyCode::Key9 && modifiers.shift() => (
                format!("{l}{}{r}", "9"),
                cursor.global_chars_offset_from_beginning + 1,
            ),
            _ if code == VirtualKeyCode::Key0 && modifiers.shift() => (
                format!("{l}{}{r}", "0"),
                cursor.global_chars_offset_from_beginning + 1,
            ),

            VirtualKeyCode::Back => (
                format!("{}{r}", &l[..l.len().saturating_sub(1)]),
                cursor.global_chars_offset_from_beginning.saturating_sub(1),
            ),
            VirtualKeyCode::Space => (
                format!("{l}{}{r}", " "),
                cursor.global_chars_offset_from_beginning + 1,
            ),

            VirtualKeyCode::Tab => (
                format!("{l}{}{r}", "    "),
                cursor.global_chars_offset_from_beginning + 4,
            ),

            VirtualKeyCode::Return | VirtualKeyCode::Escape => {
                unreachable!()
            }

            _ => (content.clone(), cursor.global_chars_offset_from_beginning),
        };
        *content = modified;
        cursor.global_chars_offset_from_beginning = new_cursor_position;
    }
}

pub fn handle_key_modification_scancode_key_press(
    content: &mut String,
    event: KeyboardInput,
    cursor: &mut Cursor,
) {
    let (l, r) = content.split_at(cursor.global_chars_offset_from_beginning);

    #[allow(deprecated)]
    if let KeyboardInput {
        scancode,
        state: ElementState::Pressed,
        modifiers,
        ..
    } = event
    {
        let (modified, new_cursor_position) = match scancode {
            41 => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "~"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "$"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }

            2 => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "%"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "&"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }

            _ if scancode == 3 && !modifiers.shift() => (
                format!("{l}{}{r}", "["),
                cursor.global_chars_offset_from_beginning + 1,
            ),

            _ if scancode == 4 && !modifiers.shift() => (
                format!("{l}{}{r}", "{"),
                cursor.global_chars_offset_from_beginning + 1,
            ),

            _ if scancode == 5 && !modifiers.shift() => (
                format!("{l}{}{r}", "}"),
                cursor.global_chars_offset_from_beginning + 1,
            ),

            _ if scancode == 6 && !modifiers.shift() => (
                format!("{l}{}{r}", "("),
                cursor.global_chars_offset_from_beginning + 1,
            ),

            _ if scancode == 7 && !modifiers.shift() => (
                format!("{l}{}{r}", "="),
                cursor.global_chars_offset_from_beginning + 1,
            ),

            _ if scancode == 8 && !modifiers.shift() => (
                format!("{l}{}{r}", "*"),
                cursor.global_chars_offset_from_beginning + 1,
            ),

            _ if scancode == 9 && !modifiers.shift() => (
                format!("{l}{}{r}", ")"),
                cursor.global_chars_offset_from_beginning + 1,
            ),

            _ if scancode == 10 && !modifiers.shift() => (
                format!("{l}{}{r}", "+"),
                cursor.global_chars_offset_from_beginning + 1,
            ),

            _ if scancode == 11 && !modifiers.shift() => (
                format!("{l}{}{r}", "]"),
                cursor.global_chars_offset_from_beginning + 1,
            ),

            _ if scancode == 12 && !modifiers.shift() => (
                format!("{l}{}{r}", "!"),
                cursor.global_chars_offset_from_beginning + 1,
            ),

            13 => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "`"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "#"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            // below numbers
            // 16, 17, 18, 26, 27, 43, 40, 44
            // :   <   >   ?   ^   |   _   "
            // ;   ,   .   /   @   \   -   '
            16 => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", ":"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", ";"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            17 => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "<"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", ","),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            18 => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", ">"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "."),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            26 => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "?"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "/"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            27 => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "^"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "@"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            43 => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "|"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "\\"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            40 => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "_"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "-"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }
            44 => {
                if modifiers.shift() {
                    (
                        format!("{l}{}{r}", "\""),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                } else {
                    (
                        format!("{l}{}{r}", "'"),
                        cursor.global_chars_offset_from_beginning + 1,
                    )
                }
            }

            _ => return,
        };
        *content = modified;
        cursor.global_chars_offset_from_beginning = new_cursor_position
    }
}

pub fn handle_key_press(content: &mut String, event: KeyboardInput, cursor: &mut Cursor) {
    if let KeyboardInput {
        virtual_keycode: Some(code),
        state: ElementState::Pressed,
        ..
    } = event
    {
        match code {
            k if KEYCODES_WHICH_MODIFY.contains(&k) => {
                handle_keycode_key_press(content, event, cursor);
            }

            _ => {}
        };
    }

    if let KeyboardInput {
        scancode: code,
        state: ElementState::Pressed,
        ..
    } = event
    {
        if SCANCODES_WHICH_MODIFY.contains(&code) {
            handle_key_modification_scancode_key_press(content, event, cursor);
        }
    }
}
