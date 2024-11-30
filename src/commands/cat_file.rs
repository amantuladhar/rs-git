use crate::log;
use crate::shared::{print_blob_obj, print_tree_obj, read_obj_file, ObjKind};
use anyhow::Context;

pub fn cat_file(pretty_print: bool, hash: String) -> anyhow::Result<()> {
    anyhow::ensure!(
        pretty_print,
        log!("only pretty print is supported at moment!!")
    );
    let obj = read_obj_file(&hash).context(log!("Unable to read object file"))?;
    match obj.kind {
        ObjKind::Blob => print_blob_obj(obj)?,
        ObjKind::Tree => print_tree_obj(obj, false)?,
        ObjKind::Commit => anyhow::bail!(log!("commit not yet implemented")),
    }
    Ok(())
}
