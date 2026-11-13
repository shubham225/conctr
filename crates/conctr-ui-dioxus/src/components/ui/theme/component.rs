/// Theme inherited by every component inside a provider.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}
