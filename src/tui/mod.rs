//! TUI (Terminal User Interface) module for KLP
//! 
//! Provides a cyberpunk-themed interface with animated dragon,
//! code generation preview, and real-time controls.

use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame, Terminal,
};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use crate::config::Config;

/// Dragon animation frames (simplified ASCII art)
const DRAGON_FRAMES: &[&str] = &[
    r#"
      \   /\   /
       )  ^^  (
      (   ||   )
       )  ||  (
      /   ||   \
     (    ||    )
      \   ||   /
       )  ||  (
    "#,
    r#"
       \  /\  /
        ) ^^ (
       (  ||  )
        ) || (
       /  ||  \
      (   ||   )
       \  ||  /
        ) || (
    "#,
    r#"
        \/  \/
        )^  ^(
       ( |  | )
        )|  |(
       / |  | \
      (  |  |  )
       \ |  | /
        )|  |(
    "#,
    r#"
        \/  \/
       ) ^^^^ (
      (   ||   )
       )  ||  (
      /   ||   \
     (    ||    )
      \   ||   /
       )  ||  (
    "#,
];

const FIRE_FRAMES: &[&str] = &[
    " 🔥 ", " 🔥🔥 ", " 🔥🔥🔥 ", " 🔥🔥 ", " 🔥 ",
];

/// Run the TUI code editor
pub async fn run_tui(file: Option<std::path::PathBuf>, config: Config) -> anyhow::Result<()> {
    info!("Starting TUI with cyberpunk theme");
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Create app state
    let mut app = App::new(config, file);
    
    // Event loop
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(1000 / 60); // 60 FPS
    
    loop {
        // Draw
        terminal.draw(|f| ui(f, &mut app))?;
        
        // Handle events
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match handle_key_event(key, &mut app).await {
                        Ok(should_exit) => {
                            if should_exit {
                                break;
                            }
                        }
                        Err(e) => {
                            app.error_message = Some(e.to_string());
                        }
                    }
                }
            }
        }
        
        // Update animation
        if last_tick.elapsed() >= Duration::from_millis(1000 / app.config.tui.dragon_frames_per_second as u64) {
            app.update_animation();
            last_tick = Instant::now();
        }
    }
    
    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    Ok(())
}

/// App state for TUI
struct App {
    config: Config,
    /// Current file being edited
    current_file: Option<std::path::PathBuf>,
    /// File content
    content: String,
    /// Current API being used
    current_api: Api,
    /// Current mode
    mode: Mode,
    /// Dragon animation frame
    dragon_frame: usize,
    /// Fire animation frame
    fire_frame: usize,
    /// Generated code preview
    generated_code: Option<String>,
    /// Error message to display
    error_message: Option<String>,
    /// User input for code generation
    input_buffer: String,
    /// Cursor position in input
    cursor_position: usize,
    /// Whether in input mode
    input_mode: bool,
    /// Scroll position for code preview
    scroll: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Api {
    Grok,
    Groq,
    OpenAI,
    Local,
}

impl Api {
    fn next(self) -> Self {
        match self {
            Api::Grok => Api::Groq,
            Api::Groq => Api::OpenAI,
            Api::OpenAI => Api::Local,
            Api::Local => Api::Grok,
        }
    }
    
    fn name(&self) -> &'static str {
        match self {
            Api::Grok => "GROK",
            Api::Groq => "GROQ",
            Api::OpenAI => "OPENAI",
            Api::Local => "LOCAL",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Mode {
    Plan,
    Execute,
}

impl Mode {
    fn toggle(self) -> Self {
        match self {
            Mode::Plan => Mode::Execute,
            Mode::Execute => Mode::Plan,
        }
    }
    
    fn name(&self) -> &'static str {
        match self {
            Mode::Plan => "PLAN",
            Mode::Execute => "EXEC",
        }
    }
}

impl App {
    fn new(config: Config, file: Option<std::path::PathBuf>) -> Self {
        let content = if let Some(ref path) = file {
            std::fs::read_to_string(path).unwrap_or_default()
        } else {
            String::new()
        };
        
        Self {
            config,
            current_file: file,
            content,
            current_api: Api::Grok,
            mode: Mode::Plan,
            dragon_frame: 0,
            fire_frame: 0,
            generated_code: None,
            error_message: None,
            input_buffer: String::new(),
            cursor_position: 0,
            input_mode: true,
            scroll: 0,
        }
    }
    
    fn update_animation(&mut self) {
        self.dragon_frame = (self.dragon_frame + 1) % DRAGON_FRAMES.len();
        self.fire_frame = (self.fire_frame + 1) % FIRE_FRAMES.len();
    }
    
    fn cyberpunk_primary(&self) -> Color {
        Color::Green
    }
    
    fn cyberpunk_secondary(&self) -> Color {
        Color::Magenta
    }
    
    fn cyberpunk_accent(&self) -> Color {
        Color::Cyan
    }
    
    fn cyberpunk_fire(&self) -> Color {
        Color::Rgb(255, 69, 0)
    }
}

async fn handle_key_event(key: event::KeyEvent, app: &mut App) -> anyhow::Result<bool> {
    use KeyCode::*;
    use KeyModifiers;
    
    match key.code {
        // Exit
        Char('x') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            return Ok(true);
        }
        Esc => {
            return Ok(true);
        }
        
        // Cycle APIs (Tab)
        Tab => {
            app.current_api = app.current_api.next();
        }
        
        // Toggle mode (Shift+Tab)
        BackTab => {
            app.mode = app.mode.toggle();
        }
        
        // Diff (Ctrl+D)
        Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.error_message = Some("Diff view not yet implemented".to_string());
        }
        
        // Run (Ctrl+R)
        Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if !app.input_buffer.is_empty() {
                // Trigger code generation
                app.generated_code = Some(format!(
                    "// Generated with {} in {} mode\n// Prompt: {}\n\nfn example() {{\n    println!(\"Hello from KLP!\");\n}}",
                    app.current_api.name(),
                    app.mode.name(),
                    app.input_buffer
                ));
            }
        }
        
        // Scroll up/down
        Up if !app.input_mode => {
            app.scroll = app.scroll.saturating_sub(1);
        }
        Down if !app.input_mode => {
            app.scroll = app.scroll.saturating_add(1);
        }
        
        // Input handling
        Char(c) => {
            app.input_buffer.insert(app.cursor_position, c);
            app.cursor_position += 1;
        }
        Backspace => {
            if app.cursor_position > 0 {
                app.cursor_position -= 1;
                app.input_buffer.remove(app.cursor_position);
            }
        }
        Left => {
            app.cursor_position = app.cursor_position.saturating_sub(1);
        }
        Right => {
            if app.cursor_position < app.input_buffer.len() {
                app.cursor_position += 1;
            }
        }
        Enter => {
            // Trigger generation on Enter
            if !app.input_buffer.is_empty() {
                app.generated_code = Some(format!(
                    "// Generated with {} in {} mode\n// Prompt: {}\n\nfn example() {{\n    println!(\"Hello from KLP!\");\n}}",
                    app.current_api.name(),
                    app.mode.name(),
                    app.input_buffer
                ));
            }
        }
        _ => {}
    }
    
    Ok(false)
}

fn ui(f: &mut Frame, app: &mut App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Header
            Constraint::Length(12), // Dragon
            Constraint::Length(3),  // Input
            Constraint::Min(10),    // Code preview
            Constraint::Length(1),  // Status bar
        ])
        .split(f.size());
    
    // Header
    let header = Paragraph::new("KLP - KALI LANGUAGE PROCESSOR v0.1.0")
        .style(Style::default().fg(app.cyberpunk_primary()).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(header, main_layout[0]);
    
    // Dragon animation area
    let dragon_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(main_layout[1]);
    
    // Dragon frame
    let dragon = Paragraph::new(DRAGON_FRAMES[app.dragon_frame])
        .style(Style::default().fg(app.cyberpunk_accent()))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(app.cyberpunk_primary())));
    f.render_widget(dragon, dragon_layout[0]);
    
    // API and mode info
    let info_text = format!(
        "🔥 {} 🔥\n\nAPI: {}\nMODE: {}\n\nShortcuts:\nTab: Switch API\nShift+Tab: Toggle Mode\nCtrl+R: Generate\nCtrl+D: Diff\nCtrl+X: Exit",
        FIRE_FRAMES[app.fire_frame],
        app.current_api.name(),
        app.mode.name()
    );
    let info = Paragraph::new(info_text)
        .style(Style::default().fg(app.cyberpunk_fire()))
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(app.cyberpunk_secondary())));
    f.render_widget(info, dragon_layout[1]);
    
    // Input area
    let input_block = Block::default()
        .title("Prompt")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.cyberpunk_primary()));
    
    let input_text = format!("{}▌", app.input_buffer);
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::White))
        .block(input_block);
    f.render_widget(input, main_layout[2]);
    
    // Code preview
    let code_content = app.generated_code.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("// Enter a prompt and press Ctrl+R to generate code");
    
    let code_block = Block::default()
        .title("Code Preview")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.cyberpunk_accent()));
    
    let code = Paragraph::new(code_content)
        .style(Style::default().fg(Color::White))
        .block(code_block)
        .wrap(Wrap { trim: true })
        .scroll((app.scroll, 0));
    f.render_widget(code, main_layout[3]);
    
    // Status bar
    let status_text = if let Some(ref error) = app.error_message {
        format!(" ERROR: {} | Press any key to dismiss ", error)
    } else {
        format!(" {} | {} | {} | Help: Ctrl+X ", 
            app.current_api.name(),
            app.mode.name(),
            app.current_file.as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "New File".to_string())
        )
    };
    
    let status_style = if app.error_message.is_some() {
        Style::default().bg(Color::Red).fg(Color::White)
    } else {
        Style::default().bg(app.cyberpunk_primary()).fg(Color::Black)
    };
    
    let status = Paragraph::new(status_text)
        .style(status_style)
        .alignment(Alignment::Left);
    f.render_widget(status, main_layout[4]);
}
