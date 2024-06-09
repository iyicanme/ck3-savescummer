use std::ops::{AddAssign, SubAssign};

use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::widgets::TableState;

use crate::save_storage::SaveStorage;
use crate::state::State;

pub struct Context {
    pub state: State,
    pub save_storage: SaveStorage,
    pub table_state: TableState,
}

impl Context {
    pub fn new() -> Self {
        Self {
            state: State::MainMenu(0, false),
            save_storage: SaveStorage::new(),
            table_state: TableState::default(),
        }
    }

    pub fn update(&mut self) {
        self.save_storage.update();
    }

    pub fn should_exit(&self) -> bool {
        self.state == State::Exit
    }

    pub fn handle_input(&mut self) -> std::io::Result<()> {
        if !event::poll(std::time::Duration::from_millis(5))? {
            return Ok(());
        }

        let Event::Key(key) = event::read()? else {
            return Ok(());
        };

        if key.kind != event::KeyEventKind::Press {
            return Ok(());
        }

        match key.code {
            KeyCode::Up => {
                self.cursor_up();
            }
            KeyCode::Down => {
                self.cursor_down();
            }
            KeyCode::Enter => {
                self.enter();
            }
            KeyCode::Esc => {
                self.exit();
            }
            _ => return Ok(()),
        };

        Ok(())
    }

    pub fn cursor_up(&mut self) {
        match &mut self.state {
            State::MainMenu(index, false) | State::SaveFileSelected(index, _, false) => {
                index.sub_assign(1);
            }
            _ => {}
        }
    }

    pub fn cursor_down(&mut self) {
        match &mut self.state {
            State::MainMenu(index, false) | State::SaveFileSelected(index, _, false) => {
                index.add_assign(1);
            }
            _ => {}
        }
    }

    pub fn enter(&mut self) {
        match self.state {
            State::Exit => {}
            State::MainMenu(index, false) => {
                self.state = State::SaveFileSelected(0, index, false);
            }
            State::MainMenu(_, true) => {
                self.state = State::Exit;
            }
            State::SaveFileSelected(index, main_menu_index, false) => {
                self.state = State::SaveFileSelected(index, main_menu_index, true);
            }
            State::SaveFileSelected(index, main_menu_index, true) => {
                let Some(save_file) = self.save_storage.save_files().nth(main_menu_index) else {
                    return;
                };

                let path = save_file.path();
                let Some(version) = self.save_storage.save_versions(path).nth(index) else {
                    return;
                };
                let Some(data) = self.save_storage.data_of(path, version.time()) else {
                    return;
                };

                let data = data.to_vec();

                self.save_storage.add_ignore_record(path.clone());

                let _ = std::fs::write(path, data);

                self.state = State::SaveFileSelected(index, main_menu_index, false);
            }
        }
    }

    pub fn exit(&mut self) {
        match self.state {
            State::Exit => {}
            State::MainMenu(index, false) => self.state = State::MainMenu(index, true),
            State::MainMenu(index, true) => self.state = State::MainMenu(index, false),
            State::SaveFileSelected(_, main_menu_index, false) => {
                self.state = State::MainMenu(main_menu_index, false);
            }
            State::SaveFileSelected(index, main_menu_index, true) => {
                self.state = State::SaveFileSelected(index, main_menu_index, false);
            }
        }
    }
}
