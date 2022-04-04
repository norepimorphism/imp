use serde::Deserialize;
use std::{fmt, fs, path::Path};


#[derive(Deserialize)]
pub struct Config {
    pub colors: Colors,
    pub prompt: Prompt,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            colors: Colors {
                prompt: ansi_term::Color::Green,
                error: ansi_term::Color::Red,
                span: ansi_term::Color::Cyan,
            },
            prompt: Prompt {
                padding: 1,
            },
        }
    }
}

impl Config {
    pub fn read(path: impl AsRef<Path>) -> Result<Self, Error> {
        let buf = fs::read(path).map_err(Error::Io)?;
        let config: Config = toml::from_slice(buf.as_slice()).map_err(Error::Toml)?;

        Ok(config)
    }
}

pub enum Error {
    Io(std::io::Error),
    Toml(toml::de::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Io(e) => e.to_string(),
                Self::Toml(e) => e.to_string(),
            },
        )
    }
}

#[derive(Deserialize)]
pub struct Colors {
    pub prompt: ansi_term::Color,
    pub error: ansi_term::Color,
    pub span: ansi_term::Color,
}

#[derive(Deserialize)]
pub struct Prompt {
    pub padding: usize,
}
