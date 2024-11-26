use anyhow::{anyhow, Context};
use flate2::bufread::ZlibDecoder;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

#[derive(Debug)]
pub enum FileType {
    Blob,
}
impl FromStr for FileType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blob" => Ok(FileType::Blob),
            _ => Err(anyhow!("Unrecognized filetype: {s}")),
        }
    }
}

pub fn cat_file(pretty_print: String) -> anyhow::Result<()> {
    let folder = &pretty_print[..2];
    let file_name = &pretty_print[2..];
    let file = File::open(format!(".git/objects/{}/{}", folder, file_name))
        .with_context(|| format!("unable to open file - {:?}", pretty_print))?;

    let reader = BufReader::new(file);
    let decoder = ZlibDecoder::new(reader);
    let mut reader = BufReader::new(decoder);

    let mut buf = Vec::new();
    reader
        .read_until(0, &mut buf)
        .expect("unable to read header part of object file");
    let (file_type, size) = std::str::from_utf8(&buf[..buf.len() - 1])
        .with_context(|| format!("Unable to read file type and size: {:?}", buf))?
        .split_once(' ')
        .ok_or_else(|| anyhow!("Header should be two parts - {:?}", buf))
        .and_then(|(f_type, size)| {
            let file_type = FileType::from_str(f_type)
                .with_context(|| format!("Invalid file type: {}", f_type))?;
            let size = size
                .parse::<u64>()
                .with_context(|| format!("Invalid size: {}", size))?;
            Ok((file_type, size))
        })?;
    match file_type {
        FileType::Blob => {
            let mut limit_reader = reader.take(size);
            let mut stdout = std::io::stdout();
            let _ = stdout.lock();
            let n = std::io::copy(&mut limit_reader, &mut stdout)
                .context("unable to copy content to stdin")?;
            anyhow::ensure!(
                n == size,
                "size written({}) doesn't match size defined({})",
                size,
                n
            );
        }
    }
    Ok(())
}