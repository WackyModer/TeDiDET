use tui::widgets::{Block, Borders, Paragraph};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Color};
use tui::Terminal;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};

use crate::mover;
use crate::util;


pub fn check_input(keyenv: Event) -> (bool, Option<KeyEvent>, bool) {
    let mut update = false;

    let mut up_h = false;
    if let event::Event::Resize(x, y ) = keyenv {
        up_h = true;
        update = true;
    }
    if let event::Event::Key(key_event) = keyenv {
        if key_event.kind == crossterm::event::KeyEventKind::Press {
            return (true, Some(key_event), up_h)
        }
    }

    (update, None, up_h)
}

pub fn handle_keybind() {

}

pub fn handle_char_inp(editor_data: &mut util::EditorDataStruct, character: char) {
    let mut curfile = &mut editor_data.files[editor_data.current_file_index as usize];
    let mut curline = &mut curfile.file_lines[curfile.cursor_y as usize];
    curline.line_text.insert((curfile.cursor_x) as usize, character);

    curfile.cursor_x += 1;
    curline.line_length += 1;
    
}

pub fn handle_remove(editor_data: &mut util::EditorDataStruct, isDel: bool) {
    let mut file_data = &mut editor_data.files[editor_data.current_file_index as usize];
    let mut curline = &mut file_data.file_lines[file_data.cursor_y as usize];

    if isDel {
        curline.line_text.remove(file_data.cursor_x as usize);
    } else {

        if curline.line_length == editor_data.settings.line_endings {
            file_data.file_lines.remove(file_data.cursor_y as usize);
            file_data.cursor_y -= 1;
            curline = &mut file_data.file_lines[file_data.cursor_y as usize];
            file_data.cursor_x = curline.line_length-editor_data.settings.line_endings;
            return;
        }
        if file_data.cursor_x == 0 && curline.line_length > editor_data.settings.line_endings {
            let cursline = file_data.file_lines[file_data.cursor_y as usize].line_text.clone();
            file_data.file_lines.remove(file_data.cursor_y as usize);
            file_data.cursor_y -= 1;
            file_data.file_lines[file_data.cursor_y as usize].line_text.pop();
            file_data.file_lines[file_data.cursor_y as usize].line_text.pop();
            file_data.file_lines[file_data.cursor_y as usize].line_text.push_str(&cursline);
            curline = &mut file_data.file_lines[file_data.cursor_y as usize];
            file_data.cursor_x = curline.line_length-editor_data.settings.line_endings;
            curline.line_length = curline.line_text.len() as i32;
            return;
        }

        curline.line_text.remove((file_data.cursor_x-1) as usize);

        file_data.cursor_x -= 1;    

        curline.line_length -= 1;
        
    }
}