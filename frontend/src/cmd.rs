pub fn is_cmd(it: &str) -> bool {
    it.starts_with(':')
}

pub fn process<'a>(interp: &imp_backend::Interp, cmd: &'a str) {
    let (name, _) = split(cmd);
    match name {
        "h" | "help" => {
            print_usage();
        }
        "a" | "print-aliases" => {
            print_interp_aliases(interp);
        }
        "v" | "print-version" => {
            print_version();
        }
        // TODO: Add moar commands!
        // TODO: Handle invalid commands.
        _ => {

        }
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
    println!("  :v, :print-version      Prints the version number.");
}

fn print_interp_aliases(interp: &imp_backend::Interp) {
    println!(
        "{}",
        interp.aliases()
            .map(|(symbol, operand)| {
                format!("{} -> {}", symbol, operand)
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn print_version() {
    println!(env!("CARGO_PKG_VERSION"));
}
