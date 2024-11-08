use core::error;
use std::collections::binary_heap::Iter;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::env;
use std::iter::Map;
use std::path::{Path, PathBuf};

use tui::backend::CrosstermBackend;
use tui::symbols::line;
use tui::text::{self, Text};
use tui::widgets::canvas::Line;
use tui::Terminal;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};

use std::convert::TryInto;

impl EditorSettings {
    pub const UNIX_ENDINGS:usize = 1;
    pub const DOS_ENDINGS:usize = 2;
}

pub struct EditorSettings {
    pub line_endings: usize,
    pub config_path: String
}

pub struct EditorDataStruct<'a> {
    pub settings: EditorSettings,
    pub files: Vec<FileDataStruct<'a>>,
    pub current_file_index: usize,
    pub term_w: u16,
    pub term_h: u16
}

pub struct FileDataStruct<'a> {
    pub file_name: &'a str,
    pub file_path: &'a str,
    pub file_lines: Vec<LineDataStruct>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub rows_scrolled: usize,
    pub x_before_dip: usize,
    pub has_dip: bool
}

pub struct LineDataStruct {
    pub line_text: String,
    pub line_length: usize,
}


pub fn detect_os_and_get_cfg_path(ed_set: &mut EditorSettings) {
    let home_dir = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap();

    let config_dir = if cfg!(target_os = "macos") {
        let version = env::var("MACOS_VERSION").unwrap_or_else(|_| "10.0".to_string());
        let parts: Vec<&str> = version.split('.').collect();

        let major_version: i32 = parts.get(0)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        if major_version < 10 {
            panic!("Pre-OSX is not supported");
        } else {
            // not pre OSX
            ed_set.line_endings = EditorSettings::UNIX_ENDINGS;
            Path::new(&home_dir).join("Library").join("Application Support").join("TeDIDET")
        }
    } else if cfg!(target_os = "linux") {
        ed_set.line_endings = EditorSettings::UNIX_ENDINGS;
        Path::new(&home_dir).join(".config").join("TeDIDET")
    } else if cfg!(target_os = "windows") {
        ed_set.line_endings = EditorSettings::DOS_ENDINGS;
        Path::new(&home_dir).join("AppData").join("Roaming").join("TeDIDET")
    } else {
        panic!("Unsupported operating system")
    };


    ed_set.config_path = config_dir.to_string_lossy().into_owned();
}

pub fn load_file_from_tx_dat_file_path(tx_dat: &mut FileDataStruct) -> io::Result<&'static str> {
    let mut file = File::open(tx_dat.file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //Ok(Box::leak(contents.into_boxed_str()))

    Ok(Box::leak(contents.into_boxed_str()))
}

pub fn update_height(ed_dat: &mut EditorDataStruct, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()>{
    let size =  terminal.size()?;
    ed_dat.term_h = size.height;
    ed_dat.term_w = size.width;
    Ok(())
}

pub fn init<'a>(tx_dat: &mut FileDataStruct, ed_set: &mut EditorSettings) -> io::Result<()> {
    detect_os_and_get_cfg_path(ed_set);
    println!("{}\n{}\n", ed_set.line_endings, ed_set.config_path);

    // Load the file into the lines
    let mut file = File::open(tx_dat.file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut split_strings: Vec<String> = Vec::new();
    if ed_set.line_endings == EditorSettings::DOS_ENDINGS {
        split_strings = contents.split("\r\n")
            .map(|s| format!("{}\r\n", s))
            .collect();
    } else if ed_set.line_endings == EditorSettings::UNIX_ENDINGS {
        split_strings = contents.split("\n")
            .map(|s| format!("{}\n", s))
            .collect();
    }    

    let mut line_vec_list: Vec<LineDataStruct> = Vec::new();


    for s in split_strings {
        line_vec_list.push(LineDataStruct {
            line_text: s.clone(),
            line_length: s.len()
        });
    }

    tx_dat.file_lines = line_vec_list;

    Ok(())
}


pub fn saveFile(text_file: &mut FileDataStruct) {
    println!("\n\n{}", text_file.file_path);
    let mut file =  File::options()
    .write(true).open(text_file.file_path.to_owned()).expect("Some error occured opening a file for writing");

    let mut all_bytes = Vec::new();
    for line in &text_file.file_lines {
        all_bytes.extend(line.line_text.as_bytes());
    }
    let byte_slice: &[u8] = &all_bytes;

    file.write(byte_slice).expect("Error writing file");
}