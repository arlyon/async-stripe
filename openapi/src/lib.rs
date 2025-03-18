use std::fmt::Debug;
use clap::Parser;

pub mod codegen;
mod components;
mod crate_inference;
mod crate_table;
pub mod crates;
mod deduplication;
mod graph;
mod object_writing;
mod overrides;
mod printable;
mod requests;
mod rust_object;
mod rust_type;
pub mod spec;
pub mod spec_fetch;
mod spec_inference;
mod stripe_object;
mod templates;
mod types;
pub mod url_finder;
pub mod utils;
mod visitor;
mod webhook;

pub const STRIPE_TYPES: &str = "stripe_types";

#[derive(Debug, Parser)]
pub struct Command {
    /// Input path for the OpenAPI spec, defaults to `spec3.sdk.json`
    #[arg(default_value = "spec3.sdk.json")]
    pub spec_path: String,
    /// Output directory for generated code, defaults to `out`
    #[arg(long, default_value = "out")]
    pub out: String,
    /// If not passed, skips the step of fetching the spec. Otherwise, `latest` for the
    /// newest spec release, `current` for the version used in the latest codegen update,
    /// or a specific version, such as `v171`
    #[arg(long, value_parser = spec_fetch::parse_spec_version)]
    pub fetch: Option<spec_fetch::SpecVersion>,
    /// Instead of writing files, generate a graph of dependencies in `graphviz` `DOT` format. Writes
    /// to `graph.txt`
    #[arg(long)]
    pub graph: bool,
    /// Update the Stripe API docs instead of using the existing data in the repo
    #[arg(long)]
    pub update_api_docs: bool,
    /// The URL to target for the stripe docs.
    #[arg(long, default_value = "https://stripe.com/docs/api")]
    pub api_docs_url: String,
    /// Skip the step of copying the generated code from `out` to `generated/`.
    #[arg(long)]
    pub dry_run: bool,
}

impl Command {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}