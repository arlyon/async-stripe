use std::{
    collections::{BTreeSet, HashSet},
    fs,
};

use anyhow::{Context, Result};
use structopt::StructOpt;

use crate::{
    crate_generator::CrateGenerator, metadata::Metadata, spec_fetch::fetch_spec,
    url_finder::UrlFinder,
};

mod codegen;
mod crate_generator;
mod file_generator;
mod mappings;
mod metadata;
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

    let v: Vec<_> =
        meta.get_crates().into_iter().map(|mut c| c.write(&out_path, &meta, &url_finder)).collect();

    // write files and get those files referenced
    // let shared_objects = meta
    //     .get_files()
    //     .into_iter()
    //     .flat_map(|mut f| f.write(&out_path, &meta, &crate_state, &url_finder))
    //     .flatten();

    // println!(
    //     "{:#?}",
    //     meta.get_files()
    //         .into_iter()
    //         .map(|mut f| {
    //             f.generate(&meta, &url_finder).unwrap();
    //             format!(
    //                 "{} - {:?} {:?} {:?} {:?}",
    //                 &f.name,
    //                 f.imported.config,
    //                 f.imported.ids,
    //                 f.imported.params,
    //                 f.imported.resources
    //             )
    //         })
    //         .collect::<HashSet<_>>()
    // );
    // println!("{:#?}", shared_objects.map(|f| f.name).collect::<HashSet<_>>());

    // write out the 'indirect' files
    // let extra_objects = shared_objects
    //     .flat_map(|mut f| f.write(&out_path, &meta, &crate_state, &url_finder))
    //     .flatten()
    //     .collect::<BTreeSet<_>>();

    // todo(arlyon): understand the implications of this
    // log::warn!("leftover objects: {:#?}", extra_objects);

    Ok(())
}
