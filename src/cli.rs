pub mod graph;

use colored::Colorize;

fn compute(x: f64) -> f64 {
    ((x * 5.0).powf(2.0) - 9.0) / ((x * 3.0).powf(2.0) + 5.0 * x + 2.0)
}

pub fn init() {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();

    if let Some(x) = args.get_mut(0) {
        match &*x.to_lowercase() {
            "graph" => {
                println!("{}", "Successfully graphed!".green().bold());
                graph::graph(compute);
            }
            "--help" | "-h" => println!("{HELP_MESSAGE}"),
            "--version" | "-v" => println!("{VERSION}"),
            "--error-test" => error("Missing argument 1.", "mathical --help"),
            _ => println!("{}: Unknown command", "ERROR:".red()),
        }
    } else {
        println!("{}", "Need first argument".red().bold());
        std::process::exit(0x0001);
    }

    //let _parsed_equation: () = parser::parse();
}

pub fn error(error_message: &str, usage: &str) {
    let colored_error: &str = "ERROR:";

    println!(
        "
{} {}

USAGE:
    {}

For more information, try the command {}
",
        colored_error.red().bold(),
        error_message,
        usage,
        "--help".green().bold()
    );
}

const VERSION: &str = "v1.0.0";
const HELP_MESSAGE: &str = "
Graphical Version 1.0.0

USAGE:
graphical <action> [arguments]

FLAGS:
    -h, --help       Prints help information
    -v, --version    Prints version information

ACTIONS:
    graph <args>     Creates a window with graph of the function or equation provided

ARGS:
    <action>         The type of action performed
    <arguments>      Arguments to the action
";
