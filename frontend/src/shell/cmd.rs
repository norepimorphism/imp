/// Determines if user input is a shell command.
pub fn is_cmd(it: &str) -> bool {
    it.starts_with(':')
}

pub fn process<'a>(cmd: &'a str) {
    let (name, _) = split(cmd);
    match name {
        "h" | "help" => {
            print_usage();
        }
        "a" | "print-aliases" => {
            print_interp_aliases();
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
    println!("  :h, :help               Displays this usage information.");
    println!("  :a, :print-aliases      Prints all defined aliases.");
}

fn print_interp_aliases() {
    todo!()
}
