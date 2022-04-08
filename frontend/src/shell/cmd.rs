// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::Shell;

/// Determines if user input is a shell command.
pub fn is_cmd(it: &str) -> bool {
    it.starts_with(':')
}

pub fn process<'a>(this: &Shell, cmd: &'a str) {
    let (name, _) = split(cmd);
    match name {
        "h" | "help" => {
            print_usage();
        }
        "a" | "aliases" => {
            print_interp_aliases();
        }
        "c" | "config" => {
            print_config(this);
        }
        // TODO: Add moar commands!
        // TODO: Handle invalid commands.
        _ => {}
    }
}

fn split(cmd: &str) -> (&str, Vec<&str>) {
    let cmd = cmd
        // Remove leading colon.
        .trim_start_matches(':')
        // Remove surrounding whitespace.
        .trim();

    cmd
        // Separate command name from arguments.
        .split_once(' ')
        // Separate arguments from each other.
        .map(|(name, args)| (name, args.split_ascii_whitespace().collect()))
        .unwrap_or_else(|| (cmd, Vec::new()))
}

fn print_usage() {
    println!("Commands:");
    println!("  :h, :help               Prints this usage information.");
    println!("  :a, :aliases            Prints all defined aliases.");
    println!("  :c, :config             Prints the current configuration.");
}

fn print_interp_aliases() {
    todo!()
}

fn print_config(this: &Shell) {
    println!("{}", toml::to_string(&this.config).unwrap());
}
