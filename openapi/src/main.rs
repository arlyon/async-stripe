use std::fmt::Debug;
use std::fs;
use std::fs::File;

use anyhow::{bail, Context, Result};
use clap::Parser;
use petgraph::dot::{Config, Dot};
use tracing::info;

use crate::codegen::CodeGen;
use crate::crate_inference::Crate;
use crate::spec::Spec;
use crate::spec_fetch::fetch_spec;
use crate::url_finder::UrlFinder;
use crate::utils::write_to_file;

mod codegen;
mod components;
mod crate_inference;
mod graph;
mod ids;
mod object_writing;
mod overrides;
mod printable;
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
mod utils;
mod visitor;
mod webhook;

#[derive(Debug, Parser)]
struct Command {
    /// Input path for the OpenAPI spec, defaults to `spec3.sdk.json`
    #[arg(default_value = "spec3.sdk.json")]
    spec_path: String,
    /// Output directory for generated code, defaults to `out`
    #[arg(long, default_value = "out")]
    out: String,
    /// If not passed, skips the step of fetching the spec. Otherwise, `latest` for the
    /// newest spec release, `current` for the version used in the latest codegen update,
    /// or a specific version, such as `v171`
    #[arg(long, value_parser = spec_fetch::parse_spec_version)]
    fetch: Option<spec_fetch::SpecVersion>,
    /// Instead of writing files, generate a graph of dependencies in `graphviz` `DOT` format. Writes
    /// to `graph.txt`
    #[arg(long)]
    graph: bool,
    /// Stub the `UrlFinder` instead of making a request to `Stripe`. Meant for use in local
    /// testing to avoid network requirement / fetch time. Will mean that no `doc_url`'s will
    /// be found.
    #[arg(long)]
    stub_url_finder: bool,
    /// Skip the step of copying the generated code from `out` to `generated/`.
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Command::parse();

    let in_path = args.spec_path;
    let out_path = args.out;
    fs::remove_dir_all(&out_path).context("could not remove out folder")?;
    fs::create_dir_all(&out_path).context("could not create out folder")?;

    info!("generating code for {} to {}", in_path, out_path);

    let spec = if let Some(version) = args.fetch {
        let raw = fetch_spec(version, &in_path)?;
        Spec::new(serde_json::from_value(raw)?)
    } else {
        let raw = File::open(in_path).context("failed to load the specfile. does it exist?")?;
        Spec::new(serde_json::from_reader(&raw).context("failed to read json from specfile")?)
    };
    info!("Finished parsing spec");

    let url_finder = if !args.stub_url_finder {
        UrlFinder::new().context("couldn't initialize url finder")?
    } else {
        UrlFinder::stub()
    };
    info!("Initialized URL finder");

    let codegen = CodeGen::new(spec, url_finder)?;

    if args.graph {
        let comp_graph = codegen.components.gen_component_dep_graph();
        let graph_txt = format!("{:?}", Dot::with_config(&comp_graph, &[Config::EdgeNoLabel]));
        write_to_file(graph_txt, "graphs/components_graph.txt")?;

        let crate_graph = codegen.components.gen_crate_dep_graph();
        let graph_txt = format!("{:?}", Dot::with_config(&crate_graph, &[Config::EdgeNoLabel]));
        write_to_file(graph_txt, "graphs/crate_graph.txt")?;
        return Ok(());
    }

    codegen.write_files()?;

    let mut fmt_cmd = std::process::Command::new("cargo");
    fmt_cmd.arg("+nightly").arg("fmt").arg("--");
    for krate in Crate::all() {
        fmt_cmd.arg(format!(
            "out/{}",
            if *krate == Crate::Types {
                format!("{}/mod.rs", krate.generated_out_path())
            } else {
                format!("{}/src/mod.rs", krate.generated_out_path())
            }
        ));
    }
    fmt_cmd.arg("out/stripe_webhook/mod.rs");

    let out = fmt_cmd.output()?;
    if !out.status.success() {
        bail!("Rustfmt failed with outputs {:?}", out);
    }

    if !args.dry_run {
        run_rsync("out/crates/", "../generated/")?;
        run_rsync("out/stripe_types/", "../stripe_types/src/generated/")?;
        run_rsync("out/stripe_webhook/", "../stripe_webhook/src/generated/")?;
    }
    Ok(())
}
// --delete-during so that generated files don't stick around when not
// generated anymore, see https://github.com/arlyon/async-stripe/issues/229
fn run_rsync(src: &str, dest: &str) -> Result<()> {
    let out = std::process::Command::new("rsync")
        .arg("-a")
        .arg("--delete-during")
        .arg(src)
        .arg(dest)
        .output()?;

    if !out.status.success() {
        bail!("rsync failed with outputs {:?}", out);
    }
    Ok(())
}
