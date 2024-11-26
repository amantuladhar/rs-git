use std::{
    fs::{self, File},
    io::{self, BufReader, Read},
};

use anyhow::Context;
use flate2::{bufread::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

use crate::shared::ToHex;

pub fn hash_object(write: Option<String>) -> anyhow::Result<()> {
    let Some(fname) = write else {
        anyhow::bail!("-w mush have filename")
    };
    // Open file to read content
    let file = File::open(&fname).with_context(|| format!("Unable to open file: {}", &fname))?;
    let mut reader = BufReader::new(file);
    let mut plain_content = Vec::new();
    let size = reader
        .read_to_end(&mut plain_content)
        .context("Unable to read from")?;

    // Create SHA-1 hash for
    let mut content: Vec<u8> = Vec::with_capacity(size + 4 + 10);
    content.extend(format!("blob {}\0", size).as_bytes());
    content.extend(plain_content);

    let mut hasher = Sha1::new();
    hasher.update(&content);
    let sha_hash = hasher.finalize().as_slice().to_hex_string();

    // Create object file
    let obj_dir = &sha_hash[..2];
    let obj_fname = &sha_hash[2..];

    // First create the directory path if it doesn't exist
    fs::create_dir_all(format!(".git/objects/{}", obj_dir))
        .context("Failed to create object directory")?;

    let file_path = format!(".git/objects/{}/{}", obj_dir, obj_fname);
    // fs::remove_file(&file_path)
    //     .context("Unable to delete existing sha1 hash to resolve permission issue")?;

    let mut file = File::create(&file_path).context("unable to create a file")?;
    let buf_read = BufReader::new(&content[..]);
    let mut z_encoder = ZlibEncoder::new(buf_read, Compression::fast());
    io::copy(&mut z_encoder, &mut file)?;

    println!("{sha_hash}");

    Ok(())
}
