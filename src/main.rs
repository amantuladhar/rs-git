use clap::{Parser, Subcommand};
use commands::prelude::*;

mod commands;
mod shared;

fn main() -> anyhow::Result<()> {
    let args = CmdArgs::parse();
    match args.command {
        Command::Init => init(),
        Command::CatFile { pretty_print, hash } => cat_file(pretty_print, hash)?,
        Command::HashObject { write } => hash_object(write)?,
        Command::LsTree { name_only, hash } => ls_tree(name_only, hash)?,
        Command::WriteTree => write_tree()?,
        Command::CommitTree {
            tree_hash,
            message,
            parent,
        } => commit_tree(tree_hash, message, parent)?,
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
        #[arg(short = 'p', long)]
        pretty_print: bool,
        hash: String,
    },
    HashObject {
        #[arg(short = 'w', value_name = "--path=<file>")]
        write: Option<String>,
    },
    LsTree {
        #[arg(long)]
        name_only: bool,
        hash: String,
    },
    WriteTree,
    CommitTree {
        tree_hash: String,

        #[arg(short = 'm')]
        message: String,

        #[arg(short = 'p')]
        parent: Option<String>,
    },
}
