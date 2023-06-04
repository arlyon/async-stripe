use std::fs;
use std::fs::File;
use std::io::Write;

use anyhow::{anyhow, Context, Result};
use structopt::StructOpt;

use crate::codegen::CodeGen;
use crate::spec::Spec;
use crate::spec_fetch::fetch_spec;
use crate::url_finder::UrlFinder;

mod codegen;
mod components;
mod graph;
mod ids;
mod object_context;
mod requests;
mod rust_object;
mod rust_type;
mod spec;
mod spec_fetch;
mod spec_inference;
mod stripe_object;
mod templates;
mod types;
mod url_finder;

#[derive(Debug, StructOpt)]
struct Command {
    /// Input path for the OpenAPI spec, defaults to `spec3.sdk.json`
    #[structopt(default_value = "spec3.sdk.json")]
    spec_path: String,
    /// Output directory for generated code, defaults to `out`
    #[structopt(long, default_value = "out")]
    out: String,
    /// If not passed, skips the step of fetching the spec. Otherwise, `latest` for the
    /// newest spec release, `current` for the version used in the latest codegen update,
    /// or a specific version, such as `v171`
    #[structopt(long, parse(try_from_str = spec_fetch::parse_spec_version))]
    fetch: Option<spec_fetch::SpecVersion>,
    /// Instead of writing files, generate a graph of dependencies in `graphviz` `DOT` format. Writes
    /// to `graph.txt`
    #[structopt(long)]
    graph: bool,
    /// Stub the `UrlFinder` instead of making a request to `Stripe`. Meant for use in local
    /// testing to avoid network requirement / fetch time. Will mean that no `doc_url`'s will
    /// be found.
    #[structopt(long)]
    stub_url_finder: bool,
    /// Skip the step of copying the generated code from `out` to `generated/`.
    #[structopt(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Command::from_args();

    let in_path = args.spec_path;
    let out_path = args.out;
    fs::remove_dir_all(&out_path).context("could not create out folder")?;
    fs::create_dir_all(&out_path).context("could not create out folder")?;

    tracing::info!("generating code for {} to {}", in_path, out_path);

    let spec = if let Some(version) = args.fetch {
        let raw = fetch_spec(version, &in_path)?;
        Spec::new(serde_json::from_value(raw)?)
    } else {
        let raw = File::open(in_path).context("failed to load the specfile. does it exist?")?;
        Spec::new(serde_json::from_reader(&raw).context("failed to read json from specfile")?)
    };
    tracing::info!("Finished parsing spec");

    let url_finder = if !args.stub_url_finder {
        UrlFinder::new().context("couldn't initialize url finder")?
    } else {
        UrlFinder::stub()
    };
    log::info!("Initialized URL finder");

    let codegen = CodeGen::new(spec, url_finder)?;

    if args.graph {
        let graph = codegen.get_graphviz_dep_graph();
        File::create("graph.txt")?.write_all(graph.as_ref())?;
        log::info!("Wrote graph to graph.txt");
        return Ok(());
    }

    codegen.write_files()?;

    let out = std::process::Command::new("cargo")
        .arg("+nightly")
        .arg("fmt")
        .arg("--")
        .arg("out/mod.rs")
        .output()?;
    if !out.status.success() {
        return Err(anyhow!("Rustfmt failed with outputs {:?}", out));
    }

    // If not a dry run, copy files from out/ to generated/.
    // --delete so that generated files don't stick around when not
    // generated anymore, see https://github.com/arlyon/async-stripe/issues/229
    if !args.dry_run {
        let out = std::process::Command::new("rsync")
            .arg("-a")
            .arg("--delete-during")
            .arg("out/")
            .arg("../src/resources/generated")
            .output()?;

        if !out.status.success() {
            return Err(anyhow!("rsync failed with outputs {:?}", out));
        }
    }
    Ok(())
}
