use std::fmt::Write as _;
use std::path::PathBuf;

use anyhow::Context;
use indoc::formatdoc;

use crate::components::{get_components, Components};
use crate::crate_table::write_crate_table;
use crate::crates::{get_crate_doc_comment, Crate, ALL_CRATES};
use crate::object_writing::{gen_obj, gen_requests};
use crate::rust_object::{ObjectKind, ObjectMetadata};
use crate::spec::Spec;
use crate::spec_inference::infer_doc_comment;
use crate::stripe_object::StripeObject;
use crate::templates::cargo_toml::gen_crate_toml;
use crate::templates::utils::write_top_level_doc_comment;
use crate::url_finder::UrlFinder;
use crate::utils::{append_to_file, write_to_file};
use crate::webhook::write_generated_for_webhooks;

pub struct CodeGen {
    pub components: Components,
    pub spec: Spec,
    pub version: String,
}

impl CodeGen {
    pub fn new(spec: Spec, url_finder: UrlFinder, version: String) -> anyhow::Result<Self> {
        let mut components = get_components(&spec)?;

        // Attach doc urls for top-level components
        for comp in components.components.values_mut() {
            if let Some(doc_url) = url_finder.url_for_object(comp.path()) {
                comp.stripe_doc_url = Some(doc_url);
            }
        }

        Ok(Self { components, spec, version })
    }

    fn write_api_version_file(&self) -> anyhow::Result<()> {
        let base_path = Crate::SHARED.get_path();
        let mut mod_rs_contents = String::new();
        let mod_rs_path = base_path.join("mod.rs");

        // Write the current API version
        let version_file_content = format!(
            "pub const VERSION: crate::ApiVersion = crate::ApiVersion::V{};",
            self.spec.version().replace(['-', '.'], "_")
        );
        write_to_file(version_file_content, base_path.join("version.rs"))?;
        let _ = writeln!(mod_rs_contents, "pub mod version;");

        append_to_file(mod_rs_contents, mod_rs_path)
    }

    fn write_components(&self) -> anyhow::Result<()> {
        for component in self.components.components.values() {
            self.write_component(component)?;
        }

        let crate_path = Crate::SHARED.get_path();
        let crate_mod_path = crate_path.join("mod.rs");
        for (ident, typ_info) in &self.components.extra_types {
            let mut out = String::new();
            let metadata = ObjectMetadata::new(ident.clone());
            self.components.write_object(&typ_info.obj, &metadata, typ_info.usage, &mut out);
            write_to_file(out, crate_path.join(format!("{}.rs", typ_info.mod_path)))?;
            append_to_file(
                format!("pub mod {0}; pub use {0}::{1};", typ_info.mod_path, ident),
                &crate_mod_path,
            )?;
        }
        Ok(())
    }

    pub fn write_files(&self) -> anyhow::Result<()> {
        self.write_crate_base()?;
        self.write_components()?;
        self.write_api_version_file()?;
        write_generated_for_webhooks(&self.components)
            .context("Could not write webhook generated code")?;
        write_crate_table(&self.components)?;
        self.write_object_info_for_testing()
    }

    fn write_crate_base(&self) -> anyhow::Result<()> {
        let crate_graph = self.components.gen_crate_dep_graph();

        for krate in &*ALL_CRATES {
            let neighbors = crate_graph.neighbors(*krate);
            let base_path = PathBuf::from(krate.generated_out_path());
            let request_features = self
                .components
                .get_crate_members(*krate)
                .into_iter()
                .filter(|c| !self.components.get(c).requests.is_empty() && *krate != Crate::SHARED)
                .map(|c| self.components.get(c).mod_path())
                .collect();

            let toml = gen_crate_toml(*krate, neighbors.collect(), request_features, &self.version);
            write_to_file(toml, base_path.join("Cargo.toml"))?;

            let lib_name = krate.lib_name();
            let doc_comment = write_top_level_doc_comment(get_crate_doc_comment(*krate));

            // We set up some things in the base `mod.rs` file:
            // 1. Without this recursion limit increase, `cargo doc` fails
            // 2. The `extern` allows generated code to use absolute paths starting with the crate name instead of `crate`
            // 3. Allow some warnings that are not currently fixed, but could be.
            let mod_rs = formatdoc! {
                r#"
            #![recursion_limit = "256"]
            #![allow(clippy::large_enum_variant)]
            #![allow(rustdoc::broken_intra_doc_links)]
            #![allow(rustdoc::invalid_html_tags)]
            #![allow(non_camel_case_types)]

            {doc_comment}
            extern crate self as {lib_name};

            miniserde::make_place!(Place);
            "#
            };

            let mod_path = base_path.join("src/mod.rs");
            write_to_file(mod_rs, &mod_path)?;

            // NB: a hack to avoid the insanely long lines generated because of _very_ long
            // type names causing `rustfmt` errors (https://github.com/rust-lang/rustfmt/issues/5315)
        }
        Ok(())
    }

    fn write_component_requests(&self, comp: &StripeObject) -> anyhow::Result<()> {
        let req_content = gen_requests(&comp.requests, &self.components);

        let req_file_content = formatdoc! {
            r#"
            use stripe_client_core::{{StripeClient, StripeBlockingClient, StripeRequest, RequestBuilder, StripeMethod}};

            {req_content}
            "#
        };
        write_to_file(req_file_content, comp.get_requests_content_path())?;
        let feature_gate = comp.mod_path();
        let new_mod_file_content = formatdoc! {
            r#"
            #[cfg(feature = "{feature_gate}")]
            mod requests;
            #[cfg(feature = "{feature_gate}")]
            pub use requests::*;
            "#
        };
        append_to_file(new_mod_file_content, comp.get_requests_module_path())
    }

    #[tracing::instrument(level = "debug", skip_all, fields(path = %comp.path()))]
    fn write_component(&self, comp: &StripeObject) -> anyhow::Result<()> {
        let struct_defs = self.gen_struct_definitions_for_component(comp);
        if !comp.has_requests() || comp.types_split_from_requests() {
            write_to_file(struct_defs, comp.get_types_content_path())?;
            append_to_file(
                format!(
                    // NB: we add doc(hidden) and doc(inline) to hide the implementation details
                    // of these being public modules so that other generated crates can import them. It
                    // just keeps the already giant rustdoc page a bit cleaner
                    "#[doc(hidden)]\npub mod {0};#[doc(inline)]\npub use {0}::*;",
                    comp.mod_path()
                ),
                comp.types_crate().get_path_to_mod(),
            )?;
        } else {
            write_to_file(struct_defs, comp.get_types_content_path())?;
            append_to_file("pub(crate) mod types;", comp.get_requests_module_path())?;
            append_to_file(
                format!("pub use {0}::types::*;", comp.mod_path()),
                comp.req_crate().get_path_to_mod(),
            )?;
        }

        if !comp.requests.is_empty() {
            append_to_file(
                format!("pub mod {};", comp.mod_path()),
                comp.req_crate().get_path_to_mod(),
            )?;
            self.write_component_requests(comp)?;
        }

        // Reexport in this crate if we wrote types to `stripe_shared` instead of the
        // component's base crate.
        if comp.types_split_from_requests() {
            append_to_file(
                format!("pub use {}::{}::*;", Crate::SHARED.lib_name(), comp.mod_path()),
                comp.req_crate().get_path_to_mod(),
            )?;
        }

        for (ident, obj) in &comp.deduplicated_objects {
            let mut out = String::new();
            let metadata = ObjectMetadata::new(ident.clone());
            self.components.write_object(&obj.object, &metadata, obj.info.usage, &mut out);
            let dst_file = match obj.info.usage.kind {
                ObjectKind::RequestParam | ObjectKind::RequestReturned => {
                    comp.get_requests_content_path()
                }
                ObjectKind::Type => comp.get_types_content_path(),
            };
            append_to_file(out, dst_file)?;
        }

        Ok(())
    }

    fn gen_struct_definitions_for_component(&self, comp: &StripeObject) -> String {
        let base_obj = comp.rust_obj();
        let schema = self.spec.get_component_schema(comp.path());
        let doc_comment = infer_doc_comment(schema, comp.stripe_doc_url.as_deref());
        let meta = ObjectMetadata::new(comp.ident().clone()).doc(doc_comment);

        gen_obj(base_obj, &meta, comp, &self.components)
    }

    fn write_object_info_for_testing(&self) -> anyhow::Result<()> {
        let mut checks = String::new();
        for (path, obj) in &self.components.components {
            if obj.object_name().is_none() {
                continue;
            }
            let krate = obj.krate().unwrap().base();
            let import_path = format!("{}::{}", krate.lib_name(), obj.ident());
            let _ = writeln!(checks, r#"check_object::<{import_path}>(resources, "{path}");"#);
        }
        let content = formatdoc! {
            r#"
            use crate::deserialization_fixture::check_object;

            pub fn check_fixtures(resources: &serde_json::Value) {{
                {checks}
            }}
            "#
        };

        write_to_file(content, "tests/mod.rs")
    }
}
