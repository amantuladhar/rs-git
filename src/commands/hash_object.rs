use crate::shared::{write_blob_object, ToHex};

pub fn hash_object(write: Option<String>) -> anyhow::Result<()> {
    let Some(fname) = write else {
        anyhow::bail!("-w mush have filename")
    };
    let raw_sha = write_blob_object(fname)?;
    println!("{}", (&raw_sha[..]).to_hex_string());
    Ok(())
}
