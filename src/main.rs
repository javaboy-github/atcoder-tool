mod contest;

use clap::{arg, command};

fn main() {
    let matches = command!("att")
        .subcommand_required(true)
        .subcommand(
            command!("download")
                .about("download the test-cases of the given contest")
                .arg(arg!([CONTEST]).help("contest name what you want to get. for example, abc300").default_value("latest"))
        ).subcommand(
            command!("submit")
                .about("submit an code")
                .arg(arg!([FILE]))
        ).subcommand(
            command!("test")
                .about("test code by test case")
                .arg(arg!([FILE]))
        ).get_matches();
    match matches.subcommand() {
        Some(("download", sub_matches)) => println!("subcommand new"),
        _ => println!("unknown subcommand")
    }
}
