use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("task title must contain 1 to 500 characters, got {length}")]
    InvalidTaskTitleLength { length: usize },

    #[error("priority must be between 1 and 4, got {value}")]
    InvalidPriority { value: i16 },

    #[error(
        "project status must be one of \
        'idea', 'active', 'paused', 'shipped' or 'abandoned', \
        got {value}"
    )]
    InvalidProjectStatus { value: String },

    #[error("invalid HLC format, expected 'ms-counter-device', got {value}")]
    InvalidHlcFormat { value: String },
}
