use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders, Paragraph};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Color};
use tui::Terminal;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use std::io;

use crate::{input, util, mover};

pub fn setup_ui() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    // Enable raw mode
    enable_raw_mode()?;
    
    // Initialize stdout
    let stdout = io::stdout(); // Take ownership of stdout
    
    // Create the Crossterm backend and Terminal
    let backend = CrosstermBackend::new(stdout); // No mutable reference, pass ownership
    let mut terminal = Terminal::new(backend)?;

    // Clear the terminal
    terminal.clear()?;

    Ok(terminal)
}

pub fn update_ui(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, ed_dat: &mut util::EditorDataStruct) -> Result<(), io::Error> {
    let tx_dat = &ed_dat.files[ed_dat.current_file_index as usize];
    terminal.draw(|f| {
        // Create a full-screen layout
        let size = f.size();
        let block = Block::default()
            .title("TUI Example")
            .borders(Borders::ALL);


        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size);


        f.render_widget(block, layout[0]);

        
        let text_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size);
        
        //let selected_lines = &tx_dat.file_lines[tx_dat.rows_scrolled..(tx_dat.rows_scrolled+ed_dat.term_h).min(tx_dat.file_lines.len())];  // 0-indexed, so 15 is at index 14
        let start_index = tx_dat.rows_scrolled.max(0) as usize; // Ensure non-negative and cast to usize
        let end_index = (tx_dat.rows_scrolled + ed_dat.term_h).min(tx_dat.file_lines.len() as i32) as usize;
        
        // Make sure end_index doesn't exceed file_lines length
        //let end_index = end_index.min(tx_dat.file_lines.len());
        //println!("{} a", end_index);

        let selected_lines = &tx_dat.file_lines[start_index..end_index];

        let text = selected_lines
            .iter()
            .map(|line| line.line_text.as_str())  // Access the `content` field
            .collect::<Vec<&str>>()
            .join("");

        let p2 = Paragraph::new(text)//tx_dat.file_lines[selected_lines].line_text.as_str())
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(p2, text_layout[0]);
        
        /*    
        let paragraph = Paragraph::new("Hello, TUI!\n")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow));
        */
        /*
        let p2 = Paragraph::new(tx_dat.file_lines[0].line_text.as_str())//input_string.as_ref())
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow));
        */
        


        // Layout to center the content

        //f.render_widget(paragraph, layout[0]);
        //f.render_widget(p2, layout[0]);

        f.set_cursor((tx_dat.cursor_x+1) as u16, (tx_dat.cursor_y+1) as u16);
    })?;

    Ok(())
}