mod cmd;
mod imp;

use crate::config::Config;
use std::io::{self, Write as _};

pub struct Shell {
    config: Config,
}

impl Shell {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Prints the shell prompt, reads user input, and executes the appropriate processor function.
    pub fn interpret_line(&self) {
        self.print_prompt();
        let user_input = Self::read_user_input();

        if cmd::is_cmd(user_input.as_str()) {
            cmd::process(self, user_input.as_str());
        } else {
            imp::process(self, user_input.as_str())
        }
    }

    fn print_prompt(&self) {
        print!(
            "{}{}",
            crate::color(
                supports_color::Stream::Stdout,
                ">".to_string(),
                ansi_term::Style::new().bold().fg(self.config.colors.prompt),
            ),
            self.prompt_padding(),
        );
    }

    fn prompt_padding(&self) -> String {
        std::iter::repeat(' ')
            .take(self.config.prompt.padding)
            .collect()
    }

    fn read_user_input() -> String {
        let _ = io::stdout().lock().flush();
        let mut input = String::new();
        // TODO: Handle error.
        let _ = io::stdin().read_line(&mut input);

        input
    }
}
