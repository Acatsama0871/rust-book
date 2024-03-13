use clap::{Command, Arg, ArgAction};

fn main() {
    let matches = Command::new("echor")
    .version("0.0.0")
    .author("Haohang Li <hli113.stevens.edu>")
    .about("rust echo")
    .arg(
        Arg::new("config")
        .value_name("TEXT")
        .help("Text input")
        .required(true)
        .index(1)
    ).arg(
        Arg::new("omit_newline")
        .short('n')
        .help("Do not print newline")
        .action(ArgAction::SetTrue)
    )
    .get_matches();

    println!("{:#?}", matches);
}
