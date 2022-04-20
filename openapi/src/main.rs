use std::{collections::BTreeSet, fs};

use anyhow::{Context, Result};

use crate::{metadata::Metadata, url_finder::UrlFinder};

mod codegen;
mod file_generator;
mod mappings;
mod metadata;
mod types;
mod url_finder;
mod util;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut args = std::env::args().skip(1);

    let in_path = args.next().unwrap_or_else(|| "spec3.json".to_string());
    let out_path = args.next().unwrap_or_else(|| "out".to_string());
    fs::create_dir_all(&out_path).context("could not create out folder")?;

    log::info!("generating code for {} to {}", in_path, out_path);

    let raw = fs::File::open(in_path).context("failed to load the specfile. does it exist?")?;
    let spec = serde_json::from_reader(&raw).context("failed to read json from specfile")?;

    let meta = Metadata::from_spec(&spec);
    let url_finder = UrlFinder::new().context("couldn't initialize url finder")?;

    meta.write_placeholders(&out_path);

    // write files and get those files referenced
    let shared_objects = meta
        .get_files()
        .into_iter()
        .flat_map(|mut f| f.write(&out_path, &meta, &url_finder))
        .flatten();

    // write out the 'indirect' files
    let extra_objects = shared_objects
        .flat_map(|mut f| f.write(&out_path, &meta, &url_finder))
        .flatten()
        .collect::<BTreeSet<_>>();

    // todo(arlyon): understand the implications of this
    log::warn!("leftover objects: {:#?}", extra_objects);

    Ok(())
}
