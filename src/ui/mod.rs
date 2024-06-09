use std::path::PathBuf;
use std::time::SystemTime;

use chrono::{DateTime, Local};
use crossterm::{
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    Terminal, widgets::{Block, Borders, TableState},
};

use crate::context::Context;
use crate::save_file::SaveFile;
use crate::save_version::SaveVersion;
use crate::state::State;
use crate::ui::table::draw;

mod color;
mod color_set;
mod popup;
mod style;
mod table;

pub fn run() -> Result<(), std::io::Error> {
    let mut context = Context::new();

    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

    while !context.should_exit() {
        context.update();

        terminal.draw(|frame| {
            ui(frame, &mut context);
        })?;
        context.handle_input()?;
    }

    disable_raw_mode()?;
    std::io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn ui(frame: &mut Frame, context: &mut Context) -> Option<()> {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());

    render_header(frame, context, main_layout[0]);

    match context.state {
        State::MainMenu(index, show_exit_confirmation) => {
            context.table_state.select(Some(index));
            let save_files = context.save_storage.save_files();
            inflate_save_files(
                frame,
                main_layout[1],
                save_files,
                index,
                &mut context.table_state,
            );

            if show_exit_confirmation {
                popup::show_exit_confirmation(frame);
            }
        }
        State::SaveFileSelected(index, main_menu_index, show_apply_confirmation) => {
            context.table_state.select(Some(index));
            let save_file = context.save_storage.save_files().nth(main_menu_index)?;
            let save_path = save_file.path();
            let save_versions = context.save_storage.save_versions(save_path);
            inflate_save_versions(
                frame,
                main_layout[1],
                save_versions,
                index,
                &mut context.table_state,
            );

            if show_apply_confirmation {
                popup::show_apply_confirmation(frame);
            }
        }
        State::Exit => {}
    }

    render_footer(frame, &context.state, main_layout[2]);

    None
}

fn render_header(frame: &mut Frame, context: &Context, area: Rect) {
    let subtitle = match context.state {
        State::SaveFileSelected(_, main_menu_index, _) => {
            let save_file = context
                .save_storage
                .save_files()
                .nth(main_menu_index)
                .unwrap_or_else(|| SaveFile::new(PathBuf::new(), SystemTime::UNIX_EPOCH));
            let file_name = save_file.path().file_name().unwrap_or_default();
            format!(" - {}", file_name.to_string_lossy())
        }
        _ => String::new(),
    };

    let header = Block::new()
        .title(format!(" CK3 Save Scummer{subtitle} "))
        .style(style::HEADER)
        .borders(Borders::TOP);

    frame.render_widget(header, area);
}

fn inflate_save_files(
    frame: &mut Frame,
    rect: Rect,
    save_files: impl Iterator<Item = SaveFile>,
    selected: usize,
    table_state: &mut TableState,
) {
    let header = ["#", "Filename", "Last Modified"];

    let rows = save_files.enumerate().filter_map(|(order, save_file)| {
        let order = format!("{order}");
        let file_name = save_file.path().file_name()?.to_string_lossy().to_string();
        let time = save_file.time();
        let time = DateTime::<Local>::from(*time);
        let time_string = time.format("%d/%m/%Y %T").to_string();

        Some([order, file_name, time_string].into_iter())
    });

    draw(frame, rect, header.into_iter(), rows, selected, table_state);
}

fn inflate_save_versions(
    frame: &mut Frame,
    rect: Rect,
    save_versions: impl Iterator<Item = SaveVersion>,
    selected: usize,
    table_state: &mut TableState,
) {
    let header = ["#", "Last Modified"];

    let rows = save_versions.enumerate().map(|(order, version)| {
        let order = format!("{order}");
        let time = version.time();
        let time = DateTime::<Local>::from(*time);
        let time_string = time.format("%d/%m/%Y %T").to_string();

        [order, time_string].into_iter()
    });

    draw(frame, rect, header.into_iter(), rows, selected, table_state);
}

fn render_footer(frame: &mut Frame, state: &State, area: Rect) {
    let title = match state {
        State::MainMenu(_, false) => {
            "[↑] Cursor Up [↓] Cursor Down [ESC] Exit [ENTER] See version history"
        }
        State::MainMenu(_, true) => "[ESC] Go back [ENTER] Exit program",
        State::SaveFileSelected(_, _, false) => {
            "[↑] Cursor Up [↓] Cursor Down [ESC] Go back to file list [ENTER] Revert to version"
        }
        State::SaveFileSelected(_, _, true) => "[ESC] Cancel revert [ENTER] Apply version",
        State::Exit => "",
    };

    let footer = Block::new()
        .title(format!(" {title} "))
        .style(style::FOOTER)
        .borders(Borders::TOP);

    frame.render_widget(footer, area);
}
