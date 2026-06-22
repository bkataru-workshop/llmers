use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmError {
    Ok,
    InvalidParam,
    Alloc,
    Http,
    Parse,
    Timeout,
    RateLimit,
    Auth,
    NotFound,
    Server,
    Unsupported,
    Cancelled,
    Thread,
    ContextLength,
}

impl fmt::Display for LlmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok => write!(f, "ok"),
            Self::InvalidParam => write!(f, "invalid_param"),
            Self::Alloc => write!(f, "alloc"),
            Self::Http => write!(f, "http"),
            Self::Parse => write!(f, "parse"),
            Self::Timeout => write!(f, "timeout"),
            Self::RateLimit => write!(f, "rate_limit"),
            Self::Auth => write!(f, "auth"),
            Self::NotFound => write!(f, "not_found"),
            Self::Server => write!(f, "server"),
            Self::Unsupported => write!(f, "unsupported"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Thread => write!(f, "thread"),
            Self::ContextLength => write!(f, "context_length"),
        }
    }
}
