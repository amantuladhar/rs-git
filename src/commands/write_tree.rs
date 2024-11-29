use crate::{
    log,
    shared::{write_blob_object, write_object, ObjKind, ToHex},
};
use std::{
    cmp::Ordering,
    ffi::OsString,
    fs::{self},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

pub fn write_tree() -> anyhow::Result<()> {
    let raw_hash = write_tree_for_path(".")?;
    println!("{}", (&raw_hash[..]).to_hex_string());
    Ok(())
}

#[derive(Debug)]
struct TreeObject<'a> {
    mode: &'a str,
    name: OsString,
    sha1: Vec<u8>,
}

impl<'a> TreeObject<'a> {
    pub fn new(mode: &'a str, name: OsString, sha1: Vec<u8>) -> anyhow::Result<Self> {
        Ok(Self { mode, sha1, name })
    }
}

pub fn write_tree_for_path<P>(p: P) -> anyhow::Result<Vec<u8>>
where
    P: AsRef<Path>,
{
    let mut collector: Vec<TreeObject> = Vec::new();

    let mut iter = fs::read_dir(p)?;
    while let Some(Ok(entry)) = iter.next() {
        let path = entry.path();
        let name = path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!(log!("unable to resolve fname")))?
            .to_owned();

        if path.is_dir() {
            // .gitignore not implemented yet, so ignoring known unwanted artifact
            if name == ".git" || name == "target" || name == ".idea" {
                continue;
            }
            let hash = write_tree_for_path(&path)?;
            collector.push(TreeObject::new("40000", name, hash)?);
            continue;
        }
        let hash = write_blob_object(&path)?;
        let mode = get_mode(&path)?;
        let tree_obj = TreeObject::new(mode, name, hash)?;
        collector.push(tree_obj);
    }
    let raw_hash = write_tree_obj(collector)?;
    Ok(raw_hash)
}

fn get_mode<'a>(path: &PathBuf) -> anyhow::Result<&'a str> {
    let mode = path.metadata()?.mode();
    let is_symlink = path.is_symlink();
    let mode = if is_symlink {
        "120000"
    }
    // https://stackoverflow.com/a/8347325
    // There are other permission but ignoring them for now
    // Check if owner has execute permission
    else if mode & 0o100 != 0 {
        "100755"
    } else {
        "100644"
    };
    Ok(mode)
}

fn write_tree_obj(mut tree_content: Vec<TreeObject>) -> anyhow::Result<Vec<u8>> {
    // sort based on how git does it - it is not normal ascii sort
    tree_content.sort_unstable_by(|a, b| {
        let a = a
            .name
            .to_str()
            .expect("unable to convert OsString to str for sorting");
        let b = b
            .name
            .to_str()
            .expect("unable to convert OsString to str for sorting");
        if a.contains(b) {
            return Ordering::Less;
        } else if b.contains(a) {
            return Ordering::Greater;
        }
        a.cmp(&b)
    });
    let mut final_content: Vec<u8> = Vec::new();
    for c in tree_content {
        final_content.extend(format!("{} {}\0", c.mode, c.name.to_str().unwrap()).as_bytes());
        final_content.extend(c.sha1)
    }
    let raw_hash = write_object(ObjKind::Tree, &final_content[..])?;
    return Ok(raw_hash);
}
