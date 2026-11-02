use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand};
use petgraph::dot::{Config, Dot};
use stripe_openapi_codegen::codegen::CodeGen;
use stripe_openapi_codegen::crates::ALL_CRATES;
use stripe_openapi_codegen::spec::Spec;
use stripe_openapi_codegen::spec_fetch::{self, SpecUpdate};
use stripe_openapi_codegen::url_finder::{UrlFinder, update_api_doc_data};
use stripe_openapi_codegen::utils::write_to_file;
use tracing::info;

#[derive(Debug, Parser)]
#[command(about = "Generate async-stripe's OpenAPI bindings and dependency graphs")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Generate bindings and install them into the repository.
    Generate(GenerateArgs),
    /// Write component and crate dependency graphs to out/graphs.
    Graph(VersionArgs),
    /// Refresh the Stripe documentation URL mapping.
    UpdateApiDocs(UpdateApiDocsArgs),
}

#[derive(Debug, Args)]
struct VersionArgs {
    /// Library version to generate. Defaults to the root workspace version.
    #[arg(long)]
    version: Option<String>,
}

#[derive(Debug, Args)]
struct GenerateArgs {
    #[command(flatten)]
    version: VersionArgs,
    /// Output directory for generated code.
    #[arg(long, default_value = "out")]
    out: PathBuf,
    /// Update the pinned spec to `latest` or a release such as `v171`.
    #[arg(long, value_parser = spec_fetch::parse_spec_update)]
    update_spec: Option<SpecUpdate>,
    /// Generate staging output without installing it into the repository.
    #[arg(long, alias = "dry-run")]
    no_install: bool,
    /// Write crate inference warnings to out/crate_warnings.md.
    #[arg(long)]
    emit_warnings: bool,
}

#[derive(Debug, Args)]
struct UpdateApiDocsArgs {
    /// Stripe API documentation URL.
    #[arg(long, default_value = "https://stripe.com/docs/api")]
    api_docs_url: String,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    match Cli::parse().command {
        Command::Generate(args) => run_generate(args),
        Command::Graph(args) => {
            let spec = Spec::new(spec_fetch::load_pinned_spec()?);
            let codegen = build_codegen(args.version, spec)?;
            prepare_output(Path::new("out"))?;
            let components = codegen.components.gen_component_dep_graph();
            write_to_file(
                format!("{:?}", Dot::with_config(&components, &[Config::EdgeNoLabel])),
                "graphs/components_graph.txt",
            )?;
            let crates = codegen.components.gen_crate_dep_graph();
            write_to_file(
                format!("{:?}", Dot::with_config(&crates, &[Config::EdgeNoLabel])),
                "graphs/crate_graph.txt",
            )
        }
        Command::UpdateApiDocs(args) => {
            let spec = Spec::new(spec_fetch::load_pinned_spec()?);
            let codegen = build_codegen(None, spec)?;
            update_api_doc_data(&args.api_docs_url, &codegen.components)
        }
    }
}

fn build_codegen(version: Option<String>, spec: Spec) -> Result<CodeGen> {
    let url_finder = UrlFinder::new().context("Could not initialize URL finder")?;
    let version = version.map_or_else(parse_workspace_version, Ok)?;
    CodeGen::new(spec, url_finder, version)
}

fn prepare_output(out: &Path) -> Result<()> {
    if out.exists() {
        fs::remove_dir_all(out).context("Could not remove output folder")?;
    }
    fs::create_dir_all(out).context("Could not create output folder")
}

fn write_crate_inference_warnings(codegen: &CodeGen) -> Result<()> {
    if !codegen.crate_inference_warnings.is_empty() {
        let mut body = String::from(
            "## Crate Inference Warnings\n\n\
             The following components could not have their crate automatically inferred \
             and were auto-assigned. Consider adding explicit path entries in \
             `openapi/gen_crates.toml` to make these assignments permanent.\n\n\
             | Component | Assigned Crate | Depended On By | Candidate Crates |\n\
             |-----------|---------------|----------------|------------------|\n",
        );
        for w in &codegen.crate_inference_warnings {
            let deps =
                w.depended_on_by.iter().map(|p| format!("`{p}`")).collect::<Vec<_>>().join(", ");
            let crates = w
                .candidate_crates
                .iter()
                .map(|c| format!("`{}`", c.base_name()))
                .collect::<Vec<_>>()
                .join(", ");
            body.push_str(&format!(
                "| `{}` | `{}` | {} | {} |\n",
                w.component,
                w.chosen_crate.base_name(),
                deps,
                crates,
            ));
        }
        body.push_str("\n### Suggested `gen_crates.toml` entries\n\n");
        body.push_str(
            "Add the following paths to the appropriate crate's `paths` array:\n\n```toml\n",
        );
        for w in &codegen.crate_inference_warnings {
            body.push_str(&format!(
                "# currently auto-assigned to \"{}\"\n\"{}\",\n",
                w.chosen_crate.base_name(),
                w.component,
            ));
        }
        body.push_str("```\n");
        write_to_file(body, "crate_warnings.md")?;
    }

    Ok(())
}

fn run_generate(args: GenerateArgs) -> Result<()> {
    let spec = Spec::new(spec_fetch::load_generation_spec(args.update_spec)?);
    let codegen = build_codegen(args.version.version, spec)?;
    prepare_output(&args.out)?;
    if args.emit_warnings {
        write_crate_inference_warnings(&codegen)?;
    }
    codegen.write_files()?;

    let mut fmt_cmd = std::process::Command::new("cargo");
    fmt_cmd.arg("+nightly").arg("fmt").arg("--");
    for krate in &*ALL_CRATES {
        fmt_cmd.arg(format!("out/{}/src/mod.rs", krate.generated_out_path()));
    }
    fmt_cmd.arg("out/async-stripe-webhook/mod.rs");
    fmt_cmd.arg("out/tests/mod.rs");

    if !args.no_install {
        info!("Formatting generated files");

        // in a rather bizarre turn of events, we actually need to run fmt _twice_
        // to get it 'settled'. otherwise it will fail formatting checks
        let _out = fmt_cmd.output()?;
        let out = fmt_cmd.output()?;
        if !out.status.success() {
            bail!("Rustfmt failed with outputs {out:?}");
        }

        info!("Copying generated files");

        let crate_paths = std::fs::read_dir("out/crates")?
            .map(|dir| dir.map(|dir| dir.path()))
            .collect::<Result<Vec<_>, _>>()?;

        let crate_paths = crate_paths
            .iter()
            .map(|e| {
                (
                    e.as_path(),
                    Path::new("../generated/")
                        .join(format!("{}/", e.file_name().unwrap().to_str().unwrap())),
                )
            })
            .chain([
                (
                    Path::new("out/async-stripe-webhook/"),
                    Path::new("../async-stripe-webhook/src/generated/").to_owned(),
                ),
                (Path::new("out/tests/"), Path::new("../tests/tests/it/generated/").to_owned()),
            ]);

        // here, we need to copy over everything we generate, deleting previous versions
        for (folder, dest) in crate_paths {
            for path in std::fs::read_dir(folder)? {
                let path = path?;
                run_rsync(path.path().as_path(), &dest)?;
            }
        }

        std::process::Command::new("cp")
            .arg("out/crate_info.md")
            .arg("../crate_info.md")
            .output()?;
    }
    Ok(())
}

fn parse_workspace_version() -> Result<String> {
    #[derive(serde::Deserialize)]
    struct Manifest {
        workspace: Workspace,
    }
    #[derive(serde::Deserialize)]
    struct Workspace {
        package: WorkspacePackage,
    }
    #[derive(serde::Deserialize)]
    struct WorkspacePackage {
        version: String,
    }
    let manifest = fs::read_to_string("../Cargo.toml")?;
    Ok(toml::from_str::<Manifest>(&manifest)?.workspace.package.version)
}

// --delete-during so that generated files don't stick around when not
// generated anymore, see https://github.com/arlyon/async-stripe/issues/229
fn run_rsync(src: &Path, dest: &Path) -> Result<()> {
    tracing::debug!("sending {} to {}", src.display(), dest.display());
    let out = std::process::Command::new("rsync")
        .arg("-a")
        .arg("--delete-during")
        .arg(src)
        .arg(dest)
        .output()?;

    if !out.status.success() {
        bail!("rsync failed with outputs {out:?}");
    }
    Ok(())
}
