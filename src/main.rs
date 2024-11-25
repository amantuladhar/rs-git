use clap::{Parser, Subcommand};
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

#[derive(Parser, Debug)]
struct CmdArgs {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init,
}

fn main() {
    let args = CmdArgs::parse();
    match args.command {
        Command::Init => init(),
    }

    // Uncomment this block to pass the first stage
    // let args: Vec<String> = env::args().collect();
    // if args[1] == "init" {
    //     fs::create_dir(".git").unwrap();
    //     fs::create_dir(".git/objects").unwrap();
    //     fs::create_dir(".git/refs").unwrap();
    //     fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    //     println!("Initialized git directory")
    // } else {
    //     println!("unknown command: {}", args[1])
    // }
}

fn init() {
    fs::create_dir(".git").expect("unable to create .git directory");
    fs::create_dir(".git/objects").expect("unable to create .git/objects");
    fs::create_dir(".git/refs").expect("unable to create .git/refs");
    fs::write(".git/HEAD", "ref: refs/heads/main\n").expect("unable to write to HEAD files");
    println!("Initialized git directory")
}
