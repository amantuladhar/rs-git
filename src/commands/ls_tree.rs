use crate::log;
use crate::shared::{print_tree_obj, read_obj_file, ObjKind};
use anyhow::Context;

pub fn ls_tree(name_only: bool, hash: String) -> anyhow::Result<()> {
    let obj = read_obj_file(&hash).context(log!("Unable to read tree hash"))?;
    match obj.kind {
        ObjKind::Tree => print_tree_obj(obj, name_only)?,
        _ => anyhow::bail!(log!("ls-tree should have hash of tree. Found {}", obj.kind)),
    }
    Ok(())
}
