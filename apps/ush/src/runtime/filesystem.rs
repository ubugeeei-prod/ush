use super::{arity, wire};
use anyhow::{Context, Result, bail, ensure};
use std::{
    fs,
    io::Write,
    os::unix::fs::{FileTypeExt, MetadataExt},
    path::Path,
};

pub(super) fn run(name: &str, args: &[String]) -> Result<String> {
    arity(
        args,
        if matches!(name, "write_bytes" | "write_atomic") {
            2
        } else {
            1
        },
    )?;
    let path = Path::new(&args[0]);
    ensure!(!args[0].is_empty(), "path must not be empty");
    match name {
        "read_link" => path_text(fs::read_link(path)?),
        "canonicalize" => path_text(fs::canonicalize(path)?),
        "read_bytes" => Ok(wire::bytes(&fs::read(path)?)),
        "write_bytes" => {
            fs::write(path, wire::read_bytes(&args[1])?)?;
            Ok(String::new())
        }
        "write_atomic" => {
            atomic_write(path, args[1].as_bytes())?;
            Ok(String::new())
        }
        "is_socket" => match fs::metadata(path) {
            Ok(metadata) => Ok(metadata.file_type().is_socket().to_string()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok("false".into()),
            Err(error) => Err(error.into()),
        },
        _ => {
            let metadata = fs::metadata(path)?;
            Ok(match name {
                "mode" => (metadata.mode() & 0o7777).to_string(),
                "owner" => metadata.uid().to_string(),
                "group" => metadata.gid().to_string(),
                "modified" => metadata.mtime().to_string(),
                "accessed" => metadata.atime().to_string(),
                "inode" => i64::try_from(metadata.ino())
                    .context("inode exceeds Int range")?
                    .to_string(),
                "links" => i64::try_from(metadata.nlink())
                    .context("link count exceeds Int range")?
                    .to_string(),
                _ => bail!("unknown filesystem operation: {name}"),
            })
        }
    }
}

fn path_text(path: std::path::PathBuf) -> Result<String> {
    path.into_os_string()
        .into_string()
        .map_err(|_| anyhow::anyhow!("path is not valid UTF-8"))
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut file = tempfile::NamedTempFile::new_in(parent)?;
    let permissions = match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_file() => Some(metadata.permissions()),
        Ok(_) => None,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };
    file.write_all(bytes)?;
    if let Some(permissions) = permissions {
        // A write may clear set-ID bits, so restore the mode after writing.
        file.as_file().set_permissions(permissions)?;
    }
    file.as_file().sync_all()?;
    file.persist(path)?;
    Ok(())
}
