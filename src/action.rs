/// Action to invoke with an intent
pub enum Action {
    Send,
    Edit,
    Chooser,
    View,
}

impl AsRef<str> for Action {
    fn as_ref(&self) -> &str {
        match self {
            Self::Send => "ACTION_SEND",
            Self::Edit => "ACTION_EDIT",
            Self::Chooser => "ACTION_CHOOSER",
            Self::View => "ACTION_VIEW",
        }
    }
}
