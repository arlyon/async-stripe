use std::{collections::BTreeSet, fs};

use anyhow::{Context, Result};
use structopt::StructOpt;

use crate::spec::Spec;
use crate::spec_fetch::fetch_spec;
use crate::{metadata::Metadata, url_finder::UrlFinder};

#[allow(clippy::too_many_arguments)]
mod codegen;
mod file_generator;
mod mappings;
mod metadata;
mod spec;
mod spec_fetch;
mod types;
mod url_finder;
mod util;

#[derive(Debug, StructOpt)]
struct Command {
    /// Input path for the OpenAPI spec, defaults to `spec3.json`
    #[structopt(default_value = "spec3.json")]
    spec_path: String,
    /// Output directory for generated code, defaults to `out`
    #[structopt(long, default_value = "out")]
    out: String,
    /// If not passed, skips the step of fetching the spec. Otherwise, `latest` for the
    /// newest spec release, `current` for the version used in the latest codegen update,
    /// or a specific version, such as `v171`
    #[structopt(long, parse(try_from_str = spec_fetch::parse_spec_version))]
    fetch: Option<spec_fetch::SpecVersion>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Command::from_args();

    let in_path = args.spec_path;
    let out_path = args.out;
    fs::create_dir_all(&out_path).context("could not create out folder")?;

    tracing::info!("generating code for {} to {}", in_path, out_path);

    let spec = if let Some(version) = args.fetch {
        let raw = fetch_spec(version, &in_path)?;
        Spec::new(serde_json::from_value(raw)?)
    } else {
        let raw = fs::File::open(in_path).context("failed to load the specfile. does it exist?")?;
        Spec::new(serde_json::from_reader(&raw).context("failed to read json from specfile")?)
    };
    tracing::info!("Finished parsing spec");

    let meta = Metadata::from_spec(&spec);
    let url_finder = UrlFinder::new().context("couldn't initialize url finder")?;

    meta.write_placeholders(&out_path);
    meta.write_version(&out_path);

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
    tracing::warn!("leftover objects: {:#?}", extra_objects);

    Ok(())
}
