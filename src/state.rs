#[derive(Clone, Eq, PartialEq)]
pub enum State {
    MainMenu(usize, bool),
    SaveFileSelected(usize, usize, bool),
    Exit,
}
