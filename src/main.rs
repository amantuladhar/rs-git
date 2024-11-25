use clap::{Parser, Subcommand};
use commands::prelude::*;

mod commands;

fn main() -> anyhow::Result<()> {
    let args = CmdArgs::parse();
    match args.command {
        Command::Init => init(),
        Command::CatFile { pretty_print } => cat_file(pretty_print)?,
        Command::HashObject { write } => hash_object(write)?,
    }
    Ok(())
}

#[derive(Parser, Debug)]
struct CmdArgs {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init,
    CatFile {
        #[clap(short = 'p', long)]
        pretty_print: String,
    },
    HashObject {
        #[clap(short = 'w')]
        write: bool,
    },
}
