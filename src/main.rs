use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use image::{DynamicImage, ImageReader};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::error::Error;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use viuer::{print, Config};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    Ok(())
}
/*
fn load_frames<P: AsRef<Path>>(dir: P) -> Result<Vec<DynamicImage>, Box<dyn Error>> {
    // Read all files in the directory
    let paths = fs::read_dir(dir);
    let mut frames: Vec<DynamicImage> = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let path = entry.path();
        let img = ImageReader::open(path).unwrap().decode().unwrap();
        frames.push(img);
    }
    Ok(frames);
}
*/
fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    // Pet animation frames
    let mut frames: Vec<DynamicImage> = Vec::new();
    for entry in fs::read_dir("assets/")? {
        let path = entry?.path();
        let img = ImageReader::open(path).unwrap().decode().unwrap();
        frames.push(img);
    }

    let mut frame_index = 0;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Length(3)])
                .split(size);

            let image_block = Block::default().title("Bloopy").borders(Borders::ALL);
            f.render_widget(image_block, chunks[0]);

            let instructions = Paragraph::new(Spans::from(vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to quit."),
            ]));
            f.render_widget(instructions, chunks[1]);

            // Now draw the image manually inside the top chunk using term-image
            let (x, y) = (chunks[0].x + 2, chunks[0].y + 1);
            let (width, height) = (chunks[0].width - 4, chunks[0].height - 2);

            let conf = Config {
                // Set dimensions.
                width: Some(width.into()),
                height: Some(height.into()),
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
                ..Default::default()
            };

            // Display `img.jpg` with dimensions 80×25 in terminal cells.
            // The image resolution will be 80×50 because each cell contains two pixels.
            print(&frames[frame_index], &conf).expect("Image printing failed.");
        })?;

        // Cycle frames
        frame_index = (frame_index + 1) % frames.len();

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    Ok(())
}
