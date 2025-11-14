#[derive(Clone, Default, PartialEq)]
pub struct Feedback {
    error: Option<String>,
    status: Option<String>,
}

impl Feedback {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            error: Some(message.into()),
            status: None,
        }
    }

    pub fn status(message: impl Into<String>) -> Self {
        Self {
            error: None,
            status: Some(message.into()),
        }
    }

    pub fn error_text(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn status_text(&self) -> Option<&str> {
        self.status.as_deref()
    }
}
