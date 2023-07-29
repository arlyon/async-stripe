use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Context;

pub fn write_to_file<C: AsRef<[u8]>, P: AsRef<Path>>(
    content: C,
    out_path: P,
) -> anyhow::Result<()> {
    let mut base = PathBuf::from("out");
    base.push(out_path);
    create_dir_all(base.parent().unwrap())?;
    File::create(&base)
        .with_context(|| format!("Could not create file at {}", base.display()))?
        .write_all(content.as_ref())?;
    Ok(())
}
