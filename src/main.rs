use crossterm::event::{self, Event, KeyCode};
use ratatui::{Frame, widgets::Paragraph};

fn main() {
    let mut terminal = ratatui::init();
    let mut input = String::new();

    loop {
        let input_clone = input.clone();
        terminal
            .draw(|frame| draw(frame, &input_clone))
            .expect("failed to draw");

        if let Event::Key(key) = event::read().expect("failed to read event") {
            match key.code {
                KeyCode::Char(c) => input.push(c),
                KeyCode::Backspace => {
                    input.pop();
                }
                KeyCode::Enter => break,
                _ => {}
            }
        }
    }

    ratatui::restore();
}

fn draw(frame: &mut Frame, input: &str) {
    let text = format!("{}\n{}", input, input);
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, frame.area());
}
