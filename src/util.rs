use core::error;
use std::collections::binary_heap::Iter;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::env;
use std::iter::Map;
use std::path::{Path, PathBuf};

use tui::backend::CrosstermBackend;
use tui::symbols::line;
use tui::text::Text;
use tui::widgets::canvas::Line;
use tui::Terminal;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};

impl EditorSettings {
    pub const UNIX_ENDINGS:i32 = 1;
    pub const DOS_ENDINGS:i32 = 2;
}

pub struct EditorSettings {
    pub line_endings: i32,
    pub config_path: String
}

pub struct EditorDataStruct<'a> {
    pub settings: EditorSettings,
    pub files: Vec<FileDataStruct<'a>>,
    pub current_file_index: i32,
    pub term_w: i32,
    pub term_h: i32
}

pub struct FileDataStruct<'a> {
    pub file_name: &'a str,
    pub file_path: &'a str,
    pub file_lines: Vec<LineDataStruct>,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub rows_scrolled: i32,
    pub x_before_dip: i32,
    pub has_dip: bool
}

pub struct LineDataStruct {
    pub line_text: String,
    pub line_length: i32,
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
    ed_dat.term_h = size.height as i32;
    ed_dat.term_w = size.width as i32;
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
            line_length: (s.len() as i32)
        });
    }

    tx_dat.file_lines = line_vec_list;

    Ok(())
}
