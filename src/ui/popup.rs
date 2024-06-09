use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Line;
use ratatui::widgets::Block;

pub fn show_exit_confirmation(frame: &mut Frame) {
    show_esc_enter_popup(
        frame,
        "Are you sure you want to exit?",
        "Go back",
        "Confirm exit",
    );
}

pub fn show_apply_confirmation(frame: &mut Frame) {
    show_esc_enter_popup(
        frame,
        "Are you sure you want to revert to this version?",
        "Go back",
        "Confirm apply",
    );
}

fn show_esc_enter_popup(
    frame: &mut Frame,
    question_message: &str,
    cancel_message: &str,
    accept_message: &str,
) {
    let question_line = Line::from(question_message).centered();
    let question_line_width = question_line.width();
    let cancel_line = Line::from("[ESC] ".to_owned() + cancel_message).centered();
    let accept_line = Line::from("[ENTER] ".to_owned() + accept_message).centered();
    let bottom_line_cell_width = cancel_line.width().max(accept_line.width());
    let line_width = u16::try_from(question_line_width.max(2 * (bottom_line_cell_width + 5)))
        .unwrap_or(u16::MAX);
    let popup_height = 5;

    let rect = frame.size();
    let area = centered_rect(line_width, popup_height, rect);

    let block = Block::bordered();
    let block_area = block.inner(area);

    let inner_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ],
    )
    .split(block_area);

    frame.render_widget(question_line, inner_layout[0]);

    let bottom_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Length(line_width),
            Constraint::Length(line_width),
        ],
    )
    .split(inner_layout[2]);
    frame.render_widget(cancel_line, bottom_layout[0]);
    frame.render_widget(accept_line, bottom_layout[1]);

    frame.render_widget(block, area);
}

fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let vertical_offset = (r.height - height) / 2;
    let popup_layout = Layout::vertical([
        Constraint::Length(vertical_offset),
        Constraint::Length(height),
        Constraint::Length(vertical_offset),
    ])
    .split(r);

    let horizontal_offset = (r.width - width) / 2;
    Layout::horizontal([
        Constraint::Length(horizontal_offset),
        Constraint::Length(width),
        Constraint::Length(horizontal_offset),
    ])
    .split(popup_layout[1])[1]
}
