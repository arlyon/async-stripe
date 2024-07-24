use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Context;

/// Write to a file, starting paths from the `out` directory and ensuring existence
/// of directories along the way.
pub fn write_to_file<C: AsRef<[u8]>, P: AsRef<Path>>(
    content: C,
    out_path: P,
) -> anyhow::Result<()> {
    let mut opts = OpenOptions::new();
    opts.truncate(true);
    write_or_append_to_outfile(content, out_path, opts)
}

/// Same as `write_to_file`, but appending instead of truncating if the file exists.
pub fn append_to_file<C: AsRef<[u8]>, P: AsRef<Path>>(
    content: C,
    out_path: P,
) -> anyhow::Result<()> {
    let mut opts = OpenOptions::new();
    opts.append(true);
    write_or_append_to_outfile(content, out_path, opts)
}

fn write_or_append_to_outfile<C: AsRef<[u8]>, P: AsRef<Path>>(
    content: C,
    out_path: P,
    mut opts: OpenOptions,
) -> anyhow::Result<()> {
    let mut base = PathBuf::from("out");
    base.push(out_path);
    create_dir_all(base.parent().unwrap())
        .with_context(|| format!("Could not create directories along path {}", base.display()))?;
    opts.create(true)
        .write(true)
        .open(&base)
        .with_context(|| format!("Could not create file at {}", base.display()))?
        .write_all(content.as_ref())?;
    Ok(())
}
