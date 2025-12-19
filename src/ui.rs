use std::vec;

use ratatui::{
    Frame, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, text::{Line, Span, Text}, widgets::{Block, Borders, Padding, Paragraph, Wrap}
};

use crate::app::{App, CurrentSection, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &App) {
    // Color palette
    let color_active = Color::Rgb(188, 126, 250);
    let color_inactive = Color::Rgb(255, 248, 173);
    let color_main_border = Color::Rgb(100, 150, 255);
    let color_footer = Color::Rgb(252, 164, 192);
    let color_background = Color::Black;

    // Render global background
    let background_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    frame.render_widget(background_block, frame.area());

    // Render main container (app border)
    let main_container = Block::default()
        .title(" ROTator ")
        .borders(Borders::ALL)
        .style(Style::default().fg(color_main_border));
    let main_inner = main_container.inner(frame.area());
    frame.render_widget(main_container, frame.area());

    // Create main layout inside the app border (content + footer)
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(main_inner);

    // Create content layout (horizontal split: input + result)
    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .split(main_layout[0]);

    // Determine active section styles
    let (input_style, result_style) = match app.current_section {
        CurrentSection::Input => (
            Style::default().fg(color_active).add_modifier(Modifier::BOLD),
            Style::default().fg(color_inactive)
        ),
        CurrentSection::Result => (
            Style::default().fg(color_inactive),
            Style::default().fg(color_active).add_modifier(Modifier::BOLD)
        ),
    };

    // Build and render input section
    let input_block = Block::default()
        .title(" Cypher ")
        .borders(Borders::ALL)
        .style(input_style);
    let input_paragraph = Paragraph::new(app.ciphertext.original_text.clone())
        .block(input_block);
    frame.render_widget(input_paragraph, content_layout[0]);

    // Build and render result section
    let result_block = Block::default()
        .title(" Results ")
        .borders(Borders::ALL)
        .style(result_style);
    let result_paragraph = Paragraph::new(app.ciphertext.current_text.clone())
        .block(result_block);
    frame.render_widget(result_paragraph, content_layout[1]);

    // Build and render footer (hint)
    let footer_block = Block::default()
        .borders(Borders::ALL)
        .title(" Hint ")
        .style(Style::default().fg(color_footer));

    frame.render_widget(footer_block, main_layout[1]);

    let footer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(9),
        ])
        .split(main_layout[1]);

    let current_key_text = vec![
        match app.current_screen {
            CurrentScreen::Main => {
                match app.current_section {
                    CurrentSection::Input => Span::styled("Typing", Style::default()),
                    CurrentSection::Result => Span::styled("ROTating", Style::default())
                }
            }
            CurrentScreen::Exiting => {
                Span::styled("Exiting", Style::default())
            }
        }
        .to_owned(),

        Span::styled(" | ", Style::default()),

        match app.current_screen {
            CurrentScreen::Main => {
                match app.current_section {
                    CurrentSection::Input => Span::styled("(Esc) to quit / (Tab) to switch section", Style::default()),
                    CurrentSection::Result => Span::styled("(Arrow Up or Down) to change ROT / (Esc) to quit / (Tab) to switch section", Style::default())
                }
            }
            CurrentScreen::Exiting => {
                Span::styled("(Esc) to cancel / (q) or (n) to quit / (y) to quit priting result", Style::default())
            }
        }
        .to_owned(),
    ];

    let key_hint_footer = Paragraph::new(Line::from(current_key_text))
        .block(Block::default().borders(Borders::ALL));

    let current_rot_text = vec![
        Span::styled("ROT: ", Style::default()),
        Span::styled(app.ciphertext.rot.to_string(), Style::default())
    ];

    let rot_hint_footer = Paragraph::new(Line::from(current_rot_text))
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(key_hint_footer, footer_layout[0]);
    frame.render_widget(rot_hint_footer, footer_layout[1]);

    if let CurrentScreen::Exiting = app.current_screen {
        let popup_block = Block::default()
            .title(" Exit ROTator ")
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .style(Style::default().bg(color_footer).fg(color_background));

        let exit_text = Text::styled(
            "Would you like to output the result cypher? (y/n)", 
            Style::default().fg(color_background),
        );

        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap {trim: false});

        let area = centered_rect(25, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}