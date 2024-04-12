use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    pub global_chars_offset_from_beginning: usize,
    pub global_lines_offset_from_beginning: usize,
    pub px_coordinates: (usize, usize),
}

fn virtual_to_char(code: VirtualKeyCode, shift: bool) -> Option<char> {
    if (VirtualKeyCode::A..=VirtualKeyCode::Z).contains(&code) {
        let askii_code = (code as u8) - (VirtualKeyCode::A as u8) + if shift { 65 } else { 97 };
        return Some(askii_code as char);
    }

    if (VirtualKeyCode::Key1..=VirtualKeyCode::Key9).contains(&code) {
        let askii_code = (code as u8) - (VirtualKeyCode::Key1 as u8) + 49;
        return Some(askii_code as char);
    }

    match code {
        VirtualKeyCode::Key0 => Some('0'),
        VirtualKeyCode::Space => Some(' '),
        VirtualKeyCode::Comma => Some(','),
        VirtualKeyCode::Period => Some('.'),
        VirtualKeyCode::Colon => Some(':'),
        VirtualKeyCode::Semicolon => Some(';'),
        VirtualKeyCode::Apostrophe => Some('\''),
        VirtualKeyCode::Backslash => Some('\\'),
        VirtualKeyCode::Slash => Some('/'),
        VirtualKeyCode::LBracket => Some('['),
        VirtualKeyCode::RBracket => Some(']'),
        VirtualKeyCode::Asterisk => Some('*'),
        VirtualKeyCode::Equals => Some('='),
        VirtualKeyCode::Minus => Some('-'),
        _ => None,
    }
}

pub fn handle_keycode_key_press(
    content: &mut String,
    code: VirtualKeyCode,
    shift: bool,
    cursor: &mut Cursor,
) {
    let mut modified = content.clone();
    let mut new_cursor_position = cursor.global_chars_offset_from_beginning;
    let (l, r) = content.split_at(cursor.global_chars_offset_from_beginning);

    if let Some(c) = virtual_to_char(code, shift) {
        modified = format!("{l}{c}{r}");
        new_cursor_position += 1;
    }
    if VirtualKeyCode::Back == code {
        modified = format!("{}{r}", &l[..l.len().saturating_sub(1)]);
        new_cursor_position = new_cursor_position.saturating_sub(1);
    }
    if VirtualKeyCode::Tab == code {
        modified = format!("{l}{}{r}", "    ");
        new_cursor_position += 4;
    }

    *content = modified;
    cursor.global_chars_offset_from_beginning = new_cursor_position;
}

pub fn handle_key_press(content: &mut String, event: KeyboardInput, cursor: &mut Cursor) {
    #[allow(deprecated)]
    if let KeyboardInput {
        virtual_keycode: Some(code),
        modifiers,
        state: ElementState::Pressed,
        ..
    } = event
    {
        handle_keycode_key_press(content, code, modifiers.shift(), cursor)
        // TODO: Scan codes too platform/keyboard layout specific? Need more general method for
        // handling more than basic keys?
    }
}
