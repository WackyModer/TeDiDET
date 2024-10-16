
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders, Paragraph};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Color};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use util::{EditorDataStruct, EditorSettings, FileDataStruct, LineDataStruct};
use std::io::{self, Cursor};
use std::process;
use std;

use crossterm::{
    ExecutableCommand, execute,
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition}
};

mod mover;
mod ui;
mod util;
mod input;

fn main() -> Result<(), io::Error> {
    let editor_settings_loc = "";
    
    let mut load_dat:FileDataStruct = FileDataStruct {
        file_path: "C:/TeDiDET/test.txt",
        file_name: "main.rs",
        file_lines: vec![
        LineDataStruct {
            line_text: String::from("Blank"),
            line_length: String::from("Blank").len() as i32
        },
        LineDataStruct {
            line_text: String::from("Blank"),
            line_length: String::from("Blank").len() as i32
        },
        LineDataStruct {
            line_text: String::from("Blank"),
            line_length: String::from("Blank").len() as i32
        }],
        cursor_x: 0,
        cursor_y: 0,
        rows_scrolled: 0,
        x_before_dip: 0,
        has_dip: false
    };
    
    let mut editor_settings = EditorSettings {
        line_endings: EditorSettings::DOS_ENDINGS,
        config_path: String::from("BAD")
    };

    util::init(&mut load_dat, &mut editor_settings)?;
    let mut term = ui::setup_ui()?;
    
    execute!(
        io::stdout(),
        //DisableBlinking,
        crossterm::cursor::SetCursorStyle::SteadyBlock
    );
    
    let mut ed_dat:EditorDataStruct = EditorDataStruct { 
        settings: editor_settings, 
        files: vec![load_dat], 
        current_file_index: 0,
        term_w: 5,
        term_h: 5
    };
    
    util::update_height(&mut ed_dat, &mut term);

    loop {
        /*terminal.draw(|f| {
            // Create a full-screen layout
            let size = f.size();
            let block = Block::default()
                .title("TUI Example")
                .borders(Borders::ALL);
                
            let paragraph = Paragraph::new("Hello, TUI!\n")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow));


            let p2 = Paragraph::new(input_string.as_ref())
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow));
            

            // Layout to center the content
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            f.render_widget(block, layout[0]);
            f.render_widget(paragraph, layout[0]);
            f.render_widget(p2, layout[0]);
        })?;

        /*
        if true { //event::poll(std::time::Duration::from_millis(500))? {
            // Read the event.
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    // Check for specific key codes, e.g., Esc to exit.
                    KeyCode::Esc => {
                        // Exit the loop if the Escape key is pressed.
                        break;
                    }
                    // Print other key presses.
                    _ => {
                        //count = key_event.code;
                    }
                }
            }
        }*/
        */
        /*match event::read()? {
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => println!("{:?}", event),
            Event::Mouse(event) => println!("{:?}", event),
            //#[cfg(feature = "bracketed-paste")]
            Event::Paste(data) => println!("{:?}", data),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }*/
        
        let mut is_updated;
        let mut inputdat = None;
        let mut update_height;
        match input::check_input(event::read()?) {
            (updated_check, Some(inpdat), updateh) => {
                update_height = updateh;
                is_updated = updated_check;
                inputdat = Some(inpdat);
            },
            (updated_check, _, updateh) => {
                update_height = updateh;
                is_updated = updated_check;
            }
            _ => {
                panic!("uhhh a bool wasn't a bool")
            }
        }
        
        if is_updated {
            if update_height {
                _ = util::update_height(&mut ed_dat, &mut term);
            }
            if let Some(inputdat) = inputdat {
                match inputdat.code {
                    // Handle the 'Enter' key
                    KeyCode::Enter => {

                    }
                    // Handle the 'Esc' key to exit
                    KeyCode::Esc => {
                        term.clear()?;
                        process::exit(0);
                    }
                    // Handle backspace to remove the last character
                    KeyCode::Backspace => {
                        input::handle_remove(&mut ed_dat, false);
                    }
                    KeyCode::Delete => {
                        input::handle_remove(&mut ed_dat, true);
                    }
                    KeyCode::Modifier(modifierKeyCode) => {
                        print!("\n\n{}aa", modifierKeyCode);
                    }
                    // Handle character input (letters, numbers, etc.)
                    KeyCode::Char(c) => {
                        input::handle_char_inp(&mut ed_dat, c);
                        //_ = ui::update_ui(&mut term, &mut ed_dat.files[ed_dat.current_file_index as usize], &mut ed_dat);

                    }
                    KeyCode::Left => {
                        mover::move_left(&mut ed_dat);
                    }
                    KeyCode::Up => {
                        mover::move_up(&mut ed_dat);
                    }
                    KeyCode::Down => {
                        mover::move_down(&mut ed_dat);
                    }
                    KeyCode::Right => {
                        mover::move_right(&mut ed_dat);
                    }
                    _ => {
                        
                    }
                }
            }
            _ = ui::update_ui(&mut term, &mut ed_dat);
        }
        // println!("{}", tx_dat.file_dat);
        
    }

    // Restore terminal to normal mode
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
