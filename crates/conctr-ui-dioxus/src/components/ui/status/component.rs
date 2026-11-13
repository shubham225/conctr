/// Status variants shared by badges and alerts.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum Status {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}
impl Status {
    pub fn class(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }
}
