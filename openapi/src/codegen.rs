use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use anyhow::Context;
use indoc::formatdoc;
use tabled::settings::Style;
use tabled::{Table, Tabled};
use tracing::debug;

use crate::components::{get_components, Components};
use crate::crate_inference::{Crate, ALL_CRATES};
use crate::object_writing::{gen_obj, gen_requests, ObjectGenInfo};
use crate::rust_object::ObjectMetadata;
use crate::spec::Spec;
use crate::spec_inference::infer_doc_comment;
use crate::stripe_object::StripeObject;
use crate::templates::cargo_toml::gen_crate_toml;
use crate::url_finder::UrlFinder;
use crate::utils::{append_to_file, write_to_file};
use crate::webhook::write_generated_for_webhooks;

pub struct CodeGen {
    pub components: Components,
    pub spec: Spec,
    pub url_finder: UrlFinder,
}

impl CodeGen {
    pub fn new(spec: Spec, url_finder: UrlFinder) -> anyhow::Result<Self> {
        let components = get_components(&spec)?;

        Ok(Self { components, spec, url_finder })
    }

    fn write_generated_for_types_crate(&self) -> anyhow::Result<()> {
        let base_path = PathBuf::from(Crate::TYPES.generated_out_path());
        let mut mod_rs_contents = String::new();
        let mod_rs_path = base_path.join("mod.rs");

        // Write the current API version
        let version_file_content = format!(
            "pub const VERSION: crate::ApiVersion = crate::ApiVersion::V{};",
            self.spec.version().replace('-', "_")
        );
        write_to_file(version_file_content, base_path.join("version.rs"))?;
        let _ = writeln!(mod_rs_contents, "pub mod version;");

        append_to_file(mod_rs_contents, mod_rs_path)?;
        Ok(())
    }

    fn write_components(&self) -> anyhow::Result<()> {
        for component in self.components.components.values() {
            debug!("Writing component {}", component.path());
            let base_crate = component.krate_unwrapped().base();
            let base_path = PathBuf::from(base_crate.generate_to());

            let crate_for_types = component.krate_unwrapped().for_types();
            let path_for_types = PathBuf::from(crate_for_types.generate_to());

            self.write_component(component, &path_for_types)?;

            let mod_path = component.mod_path();

            // Reexport in this crate if we wrote types to `stripe_types` instead of the
            // component's base crate.
            if crate_for_types == Crate::TYPES && base_crate != Crate::TYPES {
                append_to_file(
                    format!("pub use stripe_types::{mod_path}::*;"),
                    base_path.join(&mod_path).join("mod.rs"),
                )?;
                append_to_file(
                    format!("pub use stripe_types::{};", component.resource.ident()),
                    base_path.join("mod.rs"),
                )?;
            }

            if !component.requests.is_empty() {
                debug!("Writing requests for {}", component.path());
                self.write_component_requests(component, &base_path.join(&mod_path))?;
                if base_crate != crate_for_types {
                    append_to_file(format!("pub mod {mod_path};"), base_path.join("mod.rs"))?;
                }
            }
        }

        let crate_path = PathBuf::from(Crate::TYPES.generate_to());
        let crate_mod_path = crate_path.join("mod.rs");
        for (ident, typ_info) in &self.components.extra_types {
            let mut out = String::new();
            let mut metadata = ObjectMetadata::new(ident.clone(), typ_info.gen_info);
            if let Some(doc) = &typ_info.doc {
                metadata = metadata.doc(doc.clone());
            }
            self.components.write_object(&typ_info.obj, &metadata, &mut out);
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
        write_generated_for_webhooks(&self.components)
            .context("Could not write webhook generated code")?;
        self.write_crate_info()
    }

    fn write_crate_info(&self) -> anyhow::Result<()> {
        #[derive(Tabled)]
        struct CrateDisplay {
            name: String,
            krate: String,
        }

        let mut comps = vec![];
        for (_, obj) in &self.components.components {
            if obj.requests.is_empty() {
                continue;
            }
            comps.push(CrateDisplay {
                name: obj.resource.ident().to_string(),
                krate: obj.krate_unwrapped().base().name(),
            })
        }
        comps.sort_by_key(|c| c.krate.clone());
        let mut table = Table::new(comps);
        table.with(Style::markdown());
        let display = table.to_string();
        write_to_file(display, "crate_info.md")?;
        Ok(())
    }

    fn write_crate_base(&self) -> anyhow::Result<()> {
        let crate_graph = self.components.gen_crate_dep_graph();

        for krate in &*ALL_CRATES {
            let neighbors = crate_graph.neighbors(*krate);
            if *krate == Crate::TYPES {
                self.write_generated_for_types_crate()?;
            } else {
                let base_path = PathBuf::from(krate.generated_out_path());
                let request_features = self
                    .components
                    .get_crate_members(*krate)
                    .into_iter()
                    .filter(|c| !self.components.get(c).requests.is_empty())
                    .map(|c| self.components.get(c).mod_path())
                    .collect();

                let toml = gen_crate_toml(*krate, neighbors.collect(), request_features);
                write_to_file(toml, base_path.join("Cargo.toml"))?;

                // NB: a hack to avoid the insanely long lines generated because of _very_ long
                // type names causing `rustfmt` errors (https://github.com/rust-lang/rustfmt/issues/5315)
                if *krate == Crate::TREASURY {
                    write_to_file(
                        r#"
use_small_heuristics = "Max"
reorder_imports = true
group_imports = "StdExternalCrate"
error_on_line_overflow = true
max_width = 260
                    "#,
                        base_path.join(".rustfmt.toml"),
                    )?;
                }

                let crate_name = krate.name();

                // We set up some things in the base `mod.rs` file:
                // 1. Without this recursion limit increase, `cargo doc` fails
                // 2. The `extern` allows generated code to use absolute paths starting with the crate name instead of `crate`
                // 3. Allow some warnings that are not currently fixed, but could be.
                let mod_rs = formatdoc! {
                    r#"
            #![recursion_limit = "256"]
            #![allow(clippy::large_enum_variant)]
            #![allow(rustdoc::invalid_html_tags)]
            extern crate self as {crate_name};
            "#
                };

                let mod_path = base_path.join("src/mod.rs");
                write_to_file(mod_rs, &mod_path)?;
            }
        }
        Ok(())
    }

    fn write_component_requests(
        &self,
        comp: &StripeObject,
        module_path: &Path,
    ) -> anyhow::Result<()> {
        let req_content = gen_requests(&comp.requests, &self.components);
        write_to_file(req_content, module_path.join("requests.rs"))?;
        let feature_gate = comp.mod_path();
        let new_mod_file_content = formatdoc! {
            r#"
            #[cfg(feature = "{feature_gate}")]
            mod requests;
            #[cfg(feature = "{feature_gate}")]
            pub use requests::*;
            "#
        };
        append_to_file(new_mod_file_content, module_path.join("mod.rs"))
    }

    fn gen_struct_definitions_for_component(&self, comp: &StripeObject) -> String {
        let base_obj = comp.rust_obj();
        let doc_url = self.url_finder.url_for_object(comp.path());
        let schema = self.spec.get_component_schema(comp.path());
        let doc_comment = infer_doc_comment(schema, doc_url.as_deref());
        let meta =
            ObjectMetadata::new(comp.ident().clone(), ObjectGenInfo::new_deser()).doc(doc_comment);

        gen_obj(base_obj, &meta, comp, &self.components)
    }

    #[tracing::instrument(level = "debug", skip_all, fields(path = %comp.path()))]
    fn write_component(&self, comp: &StripeObject, base_path: &Path) -> anyhow::Result<()> {
        let module_path = base_path.join(comp.mod_path());
        let obj_module_path = module_path.join("mod.rs");
        let parent_mod_path = base_path.join("mod.rs");

        write_to_file(self.gen_struct_definitions_for_component(comp), obj_module_path)?;
        append_to_file(
            format!("pub mod {0};pub use {0}::{1};", comp.mod_path(), comp.ident()),
            parent_mod_path,
        )?;

        Ok(())
    }
}
