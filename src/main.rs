extern crate ratatui;

use std::io::{self};
use std::process::{Command, Stdio};
use std::env;

use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Layout, Constraint, Direction};
use ratatui::text::Span;
use crossterm::event::{self, KeyCode, Event};

mod shell_commands;

fn main() -> Result<(), io::Error> {
    disable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();
    let mut output = String::new();

    loop{
        if let Err(e) = terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default().direction(Direction::Vertical).constraints([Constraint::Min(3), Constraint::Percentage(90)].as_ref()).split(size);

            let current_dir = match env::current_dir() {
                Ok(path) => path.display().to_string(),
                Err(e) => format!("Error Getting Current Directory: {}", e)
            };

            if chunks.len() > 0{
                let input_block = Paragraph::new(Span::from(format!("{} >{}", current_dir, input))).block(Block::default().borders(Borders::ALL).title("Input"));
                f.render_widget(input_block, chunks[0]);
    
                let output_block = Paragraph::new(Span::from(output.clone())).block(Block::default().borders(Borders::ALL).title("Output"));
                f.render_widget(output_block, chunks[1]);
            } else {
                let fallback_block = Paragraph::new("Not enough space to render input/output").block(Block::default().borders(Borders::ALL));
                f.render_widget(fallback_block, size);
            }

            ()
        }) {
            eprintln!("Error drawing terminal: {}", e);
        };

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code{
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        let trimmed_input = input.trim();

                        if trimmed_input == "exit" {
                            break;
                        }

                        if shell_commands::is_command_in_shell(trimmed_input) {
                            shell_commands::handle_shell_commands(trimmed_input, &[]);
                        } else {
                            let parts: Vec<&str> = trimmed_input.split_whitespace().collect();

                            if parts.is_empty() {
                                output = "No command entered.".to_string();
                            } else {
                                let command = parts[0];
                                let args: Vec<&str> = parts[1..].to_vec();

                                match Command::new(command).args(&args).stdout(Stdio::piped()).output() {
                                    Ok(output_res) => {
                                        output = String::from_utf8_lossy(&output_res.stdout).to_string();
                                    }
                                    Err(e) => {
                                        output = format!("Failed to execute command: {}", e)
                                    }
                                }
                            }
                        }
                        input.clear();
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    enable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
