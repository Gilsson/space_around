#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum State {
    #[default]
    LoadMenu,
    MainMenu,
    LoadGame,
    InGame,
    AbilityMenu,
}
