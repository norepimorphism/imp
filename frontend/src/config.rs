// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use std::{fmt, fs, path::Path};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub output: Output,
    pub prompt: Prompt,
    pub spans: Spans,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output: Output {
                color: ansi_term::Color::Yellow,
            },
            prompt: Prompt {
                color: ansi_term::Color::Green,
                padding: 1
            },
            spans: Spans {
                color: ansi_term::Color::Cyan,
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

#[derive(Deserialize, Serialize)]
pub struct Output {
    pub color: ansi_term::Color,
}

#[derive(Deserialize, Serialize)]
pub struct Prompt {
    pub color: ansi_term::Color,
    pub padding: usize,
}

#[derive(Deserialize, Serialize)]
pub struct Spans {
    pub color: ansi_term::Color,
}
