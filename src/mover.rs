

use crate::{input, util};


pub fn move_up(ed_dat: &mut util::EditorDataStruct) {
    let mut tx_dat = &mut ed_dat.files[ed_dat.current_file_index];
    if tx_dat.cursor_y > 0 {
        tx_dat.cursor_y -= 1;

        if tx_dat.has_dip {
            tx_dat.cursor_x = tx_dat.x_before_dip.min(tx_dat.file_lines[tx_dat.cursor_y].line_length - ed_dat.settings.line_endings);
            if tx_dat.cursor_x == tx_dat.x_before_dip {
                tx_dat.has_dip = false
            }
        }

        if tx_dat.cursor_x > tx_dat.file_lines[tx_dat.cursor_y].line_length - ed_dat.settings.line_endings {
            tx_dat.x_before_dip = tx_dat.cursor_x;
            tx_dat.cursor_x = tx_dat.file_lines[tx_dat.cursor_y].line_length - ed_dat.settings.line_endings;

            tx_dat.has_dip = true;
        }
    }
}

pub fn move_down(ed_dat: &mut util::EditorDataStruct) {
    let mut tx_dat = &mut ed_dat.files[ed_dat.current_file_index];
    if tx_dat.file_lines.len() > tx_dat.cursor_y {
        tx_dat.cursor_y += 1;

        

        if tx_dat.has_dip {
            tx_dat.cursor_x = tx_dat.x_before_dip.min(tx_dat.file_lines[tx_dat.cursor_y].line_length - ed_dat.settings.line_endings);
            if tx_dat.cursor_x == tx_dat.x_before_dip {
                tx_dat.has_dip = false
            }
        }

        if tx_dat.cursor_x > tx_dat.file_lines[tx_dat.cursor_y].line_length - ed_dat.settings.line_endings {
            tx_dat.x_before_dip = tx_dat.cursor_x;
            tx_dat.cursor_x = tx_dat.file_lines[tx_dat.cursor_y].line_length - ed_dat.settings.line_endings;

            tx_dat.has_dip = true;
        }
    }
}

pub fn move_left(ed_dat: &mut util::EditorDataStruct) {
    let mut tx_dat = &mut ed_dat.files[ed_dat.current_file_index];
    tx_dat.has_dip = false;
    if tx_dat.cursor_x > 0 {
        tx_dat.cursor_x -= 1;
    } else {
        if tx_dat.cursor_y > 0 {
            tx_dat.cursor_y -= 1;
            tx_dat.cursor_x = tx_dat.file_lines[tx_dat.cursor_y].line_length - ed_dat.settings.line_endings;
        }
    }
}

pub fn move_right(ed_dat: &mut util::EditorDataStruct) {
    let mut tx_dat = &mut ed_dat.files[ed_dat.current_file_index];
    tx_dat.has_dip = false;

    if tx_dat.cursor_x < tx_dat.file_lines[tx_dat.cursor_y].line_length - ed_dat.settings.line_endings {
        tx_dat.cursor_x += 1;
    } else {
        if tx_dat.cursor_y < tx_dat.file_lines.len() {
            tx_dat.cursor_y += 1;
            tx_dat.cursor_x = 0;
        }
    }
}