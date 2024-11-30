use chrono::prelude::*;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::shared::{read_obj_file, write_object, ObjKind, ToHex};

pub fn commit_tree(
    tree_hash: String,
    message: String,
    parent: Option<String>,
) -> anyhow::Result<()> {
    // 2fe980750b48549a635e0a504bf0525ed4e49e79
    // println!("TreeHash == {tree_hash}");
    // println!("message == {message}");
    // println!("parent== {parent:?}");

    let Ok(obj_file) = read_obj_file(&tree_hash) else {
        eprintln!("Couldn't find the tree-hash object for {tree_hash}");
        std::process::exit(1);
    };
    if obj_file.kind != ObjKind::Tree {
        eprintln!("Provided hash is not tree. Found {}", obj_file.kind);
        std::process::exit(1);
    }

    let mut commit_tree: Vec<u8> = Vec::new();
    commit_tree.extend(format!("tree {tree_hash}\n").as_bytes());
    if let Some(parent_hash) = parent {
        commit_tree.extend(format!("parent {parent_hash}\n").as_bytes());
    }
    let author_name = "Test User";
    let author_email = "test@email.com";
    let date_time: DateTime<Local> = Local::now();
    let timestamp = format!("{}", date_time.format("%s %z"));
    commit_tree.extend(format!("author {author_name} <{author_email}> {timestamp}\n").as_bytes());
    commit_tree
        .extend(format!("committer {author_name} <{author_email}> {timestamp}\n").as_bytes());
    commit_tree.extend(format!("\n").as_bytes());
    commit_tree.extend(message.as_bytes());
    commit_tree.extend(format!("\n").as_bytes());

    let raw_sha = write_object(ObjKind::Commit, &commit_tree[..])?;
    println!("{}", (&raw_sha[..]).to_hex_string());

    Ok(())
}
