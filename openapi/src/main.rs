use std::{collections::BTreeSet, fs};

use anyhow::{Context, Result};
use clap::Parser;

use crate::spec_fetch::fetch_spec;
use crate::{metadata::Metadata, url_finder::UrlFinder};

mod codegen;
mod file_generator;
mod mappings;
mod metadata;
mod spec_fetch;
mod types;
mod url_finder;
mod util;

#[derive(Debug, Parser)]
struct Command {
    #[clap(
        default_value_t = String::from("spec3.json"),
        help = "Input path for the OpenAPI spec, defaults to `spec3.json`"
    )]
    spec_path: String,
    #[clap(
        long,
        default_value_t = String::from("out"),
        help = "Output directory for generated code, defaults to `out`")
    ]
    out: String,
    #[clap(
        long,
        parse(try_from_str = spec_fetch::parse_spec_version),
        help = "If not passed, skips the step of fetching the spec. \
        Otherwise, `latest` for the newest spec release, `current` for the version \
        used in the latest codegen update, or a specific version, such as `v171`"
    )]
    fetch: Option<spec_fetch::SpecVersion>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Command::parse();

    let in_path = args.spec_path;
    let out_path = args.out;
    fs::create_dir_all(&out_path).context("could not create out folder")?;

    log::info!("generating code for {} to {}", in_path, out_path);

    let spec = if let Some(version) = args.fetch {
        fetch_spec(version, &in_path)?
    } else {
        let raw = fs::File::open(in_path).context("failed to load the specfile. does it exist?")?;
        serde_json::from_reader(&raw).context("failed to read json from specfile")?
    };

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
