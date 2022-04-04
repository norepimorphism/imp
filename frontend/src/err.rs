use std::fmt;

pub enum FrontendError {
    Args(crate::args::Error),
    Config(crate::config::Error),
}

impl fmt::Display for FrontendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}[{}]: {}",
            crate::color(
                supports_color::Stream::Stderr,
                "fatal-error".to_string(),
                ansi_term::Style::new().bold().fg(ansi_term::Color::Red),
            ),
            match self {
                Self::Args(_) => "args",
                Self::Config(_) => "config",
            },
            match self {
                Self::Args(e) => e.to_string(),
                Self::Config(e) => e.to_string(),
            }
        )
    }
}

impl FrontendError {
    pub fn exit_code(&self) -> u8 {
        match self {
            Self::Args(_) => 1,
            Self::Config(_) => 2,
        }
    }
}

pub struct BackendError {
    pub stage: Stage,
    pub inner: imp_backend::Error,
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}[{}]: {}",
            crate::color(
                supports_color::Stream::Stderr,
                "error".to_string(),
                ansi_term::Style::new().fg(ansi_term::Color::Red),
            ),
            self.stage,
            self.inner,
        )
    }
}

pub enum Stage {
    A,
    B,
    C,
    D,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::A => 'a',
            Self::B => 'b',
            Self::C => 'c',
            Self::D => 'd',
        })
    }
}
