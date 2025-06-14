use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{bail, Context, Result};
use clap::Parser;
use petgraph::dot::{Config, Dot};
use stripe_openapi_codegen::codegen::CodeGen;
use stripe_openapi_codegen::crates::ALL_CRATES;
use stripe_openapi_codegen::spec::Spec;
use stripe_openapi_codegen::spec_fetch;
use stripe_openapi_codegen::spec_fetch::fetch_spec;
use stripe_openapi_codegen::url_finder::{update_api_doc_data, UrlFinder};
use stripe_openapi_codegen::utils::write_to_file;
use tracing::info;

#[derive(Debug, Parser)]
struct Command {
    /// Version of the library to generate. If not provided, uses the version found in the root
    /// workspace.
    #[arg(long)]
    version: Option<String>,
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
    /// Update the Stripe API docs instead of using the existing data in the repo
    #[arg(long)]
    update_api_docs: bool,
    /// The URL to target for the stripe docs.
    #[arg(long, default_value = "https://stripe.com/docs/api")]
    api_docs_url: String,
    /// Skip the step of copying the generated code from `out` to `generated/`.
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Command::parse();

    let in_path = args.spec_path;
    let out_path = Path::new(&args.out);
    if out_path.exists() {
        fs::remove_dir_all(out_path).context("could not remove out folder")?;
    }
    fs::create_dir_all(out_path).context("could not create out folder")?;

    info!("generating code for {in_path} to {}", out_path.display());

    let spec = if let Some(version) = args.fetch {
        let raw = fetch_spec(version, &in_path)?;
        Spec::new(serde_json::from_value(raw)?)
    } else {
        let raw = File::open(in_path).context("failed to load the specfile. does it exist?")?;
        Spec::new(serde_json::from_reader(&raw).context("failed to read json from specfile")?)
    };
    info!("Finished parsing spec");

    let url_finder = UrlFinder::new().context("couldn't initialize url finder")?;
    let version =
        if let Some(version) = args.version { version } else { parse_workspace_version()? };
    info!("Generating with version {version}");
    let codegen = CodeGen::new(spec, url_finder, version)?;

    if args.update_api_docs {
        update_api_doc_data(&args.api_docs_url, &codegen.components)?;
        return Ok(());
    }

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
    for krate in &*ALL_CRATES {
        fmt_cmd.arg(format!("out/{}/src/mod.rs", krate.generated_out_path()));
    }
    fmt_cmd.arg("out/async-stripe-webhook/mod.rs");
    fmt_cmd.arg("out/tests/mod.rs");

    if !args.dry_run {
        info!("Formatting generated files");
        let out = fmt_cmd.output()?;
        if !out.status.success() {
            bail!("Rustfmt failed with outputs {:?}", out);
        }

        info!("Copying generated files");
        run_rsync("out/crates/", "../generated/")?;
        run_rsync("out/async-stripe-webhook/", "../async-stripe-webhook/src/generated/")?;
        run_rsync("out/tests/", "../tests/tests/it/generated/")?;

        std::process::Command::new("cp")
            .arg("out/crate_info.md")
            .arg("../crate_info.md")
            .output()?;
    }
    Ok(())
}

fn parse_workspace_version() -> Result<String> {
    let reader = BufReader::new(File::open("../Cargo.toml")?);
    for line in reader.lines() {
        let line = line?;
        let line = line.trim_ascii();
        if line.starts_with("version") {
            let version = line.split_once('=').unwrap().1.trim_ascii();
            return Ok(version.replace('"', ""));
        }
    }
    bail!("No version key found");
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
