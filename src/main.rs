mod app;
mod ui;
use app::App;
use ratatui::{Terminal, crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event,KeyCode, KeyEventKind}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}}, prelude::{Backend, CrosstermBackend}};

use std::{error::Error, io};

use crate::app::{CurrentScreen, CurrentSection};
use crate::ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_cypher()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match app.current_section {
                    CurrentSection::Input => match key.code {
                        KeyCode::Tab => { app.toggle_section(); }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Exiting;
                        }
                        KeyCode::Char(value) => {
                            app.ciphertext.push(value);
                        }
                        KeyCode::Backspace => {
                            app.ciphertext.pop();
                        }
                        _ => {}
                    }
                    CurrentSection::Result => match key.code {
                        KeyCode::Up => { app.ciphertext.rotate(1); }
                        KeyCode::Down => { app.ciphertext.rotate(25); }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Exiting;
                        }
                        KeyCode::Tab => {
                            app.toggle_section();
                        }
                        _ => {}
                    }
                }
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    },
                    KeyCode::Char('y')| KeyCode::Char('Y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') | KeyCode::Char('N') | KeyCode::Char('Q') => {
                        return Ok(false)
                    }
                    _ => {}                     
                }
            }
        }
    }
}