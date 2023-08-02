use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use indoc::formatdoc;

use crate::components::{build_field_overrides, get_components, Components, Module, Overrides};
use crate::crate_inference::Crate;
use crate::object_context::{gen_obj, gen_requests, ObjectGenInfo};
use crate::rust_object::ObjectMetadata;
use crate::spec::Spec;
use crate::spec_inference::infer_doc_comment;
use crate::stripe_object::StripeObject;
use crate::templates::cargo_toml::gen_crate_toml;
use crate::templates::derives::Derives;
use crate::url_finder::UrlFinder;
use crate::utils::{append_to_file, write_to_file};

pub struct CodeGen {
    pub components: Components,
    pub spec: Spec,
    pub url_finder: UrlFinder,
    pub overrides: Overrides,
}

impl CodeGen {
    pub fn new(spec: Spec, url_finder: UrlFinder) -> anyhow::Result<Self> {
        let overrides = build_field_overrides(&spec)?;
        let mut components = get_components(&spec)?;
        components.apply_overrides(&overrides);

        Ok(Self { components, spec, url_finder, overrides })
    }

    fn write_generated_for_types_crate(&self) -> anyhow::Result<()> {
        let base_path = PathBuf::from(Crate::Types.generated_out_path());
        let mut mod_rs_contents = String::new();
        let mod_rs_path = base_path.join("mod.rs");

        for (obj, meta) in &self.overrides.overrides {
            let mut out = String::new();
            self.components.write_object(
                obj,
                &ObjectMetadata::new(meta.ident.clone()).doc(meta.doc.clone()),
                ObjectGenInfo::new(Derives::new_deser()),
                &mut out,
            );
            write_to_file(out, base_path.join(format!("{}.rs", meta.mod_path)))?;
            let _ = writeln!(
                mod_rs_contents,
                "pub mod {0}; pub use {0}::{1};",
                meta.mod_path, meta.ident
            );
        }

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

    fn write_modules(&self) -> anyhow::Result<()> {
        for module in self.components.modules.values() {
            let krate = module.krate_unwrapped().for_types();
            let crate_path = PathBuf::from(krate.generate_to());

            let requests_crate = module.krate_unwrapped().base();
            let requests_path = PathBuf::from(requests_crate.generate_to());
            match module {
                Module::Package { name, members, .. } => {
                    append_to_file(format!("pub mod {name};"), crate_path.join("mod.rs"))?;
                    let package_mod_path = crate_path.join(name);
                    for inner_path in members {
                        let obj = self.components.get(inner_path);
                        self.write_component(obj, &package_mod_path)?;
                        if !obj.requests.is_empty() {
                            let req_path = requests_path.join(name).join(obj.mod_path());
                            self.write_component_requests(obj, &req_path)?;
                        }
                    }
                }
                Module::Component { path, .. } => {
                    let obj = self.components.get(path);
                    self.write_component(obj, &crate_path)?;

                    if !obj.requests.is_empty() {
                        let obj_mod_path = obj.mod_path();
                        self.write_component_requests(obj, &requests_path.join(&obj_mod_path))?;
                        if krate != requests_crate {
                            append_to_file(
                                format!("pub mod {obj_mod_path};"),
                                requests_path.join("mod.rs"),
                            )?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn write_files(&self) -> anyhow::Result<()> {
        self.write_crate_base()?;
        self.write_modules()
    }

    fn write_crate_base(&self) -> anyhow::Result<()> {
        let crate_graph = self.components.gen_crate_dep_graph();

        for krate in Crate::all() {
            let neighbors = crate_graph.neighbors(*krate);
            if *krate == Crate::Types {
                self.write_generated_for_types_crate()?;
            } else {
                let base_path = PathBuf::from(krate.generated_out_path());

                let toml = gen_crate_toml(*krate, neighbors.collect());
                write_to_file(toml, base_path.join("Cargo.toml"))?;

                let crate_name = krate.name();

                // We set up 2 things in the base `mos.rs` file:
                // 1. Without this recursion limit increase, `cargo doc` fails
                // 2. The `extern` allows generated code to use absolute paths starting with the crate name instead of `crate`
                let mod_rs = formatdoc! {
                    r#"
            #![recursion_limit = "256"]
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
        append_to_file("pub mod requests;", module_path.join("mod.rs"))
    }

    fn gen_struct_definitions_for_component(&self, comp: &StripeObject) -> String {
        let base_obj = comp.rust_obj();
        let doc_url = self.url_finder.url_for_object(comp.path());
        let schema = self.spec.get_component_schema(comp.path());
        let doc_comment = infer_doc_comment(schema, doc_url.as_deref());
        let meta = ObjectMetadata::new(comp.ident().clone()).doc(doc_comment);

        gen_obj(base_obj, &meta, comp, &self.components)
    }

    fn write_component(&self, comp: &StripeObject, base_path: &Path) -> anyhow::Result<()> {
        let module_path = base_path.join(comp.mod_path());
        let obj_module_path = module_path.join("mod.rs");
        let parent_mod_path = base_path.join("mod.rs");

        write_to_file(self.gen_struct_definitions_for_component(comp), &obj_module_path)?;

        append_to_file(
            format!("pub mod {0};pub use {0}::{1};", comp.mod_path(), comp.ident()),
            &parent_mod_path,
        )?;

        let deleted_component_path = format!("deleted_{}", comp.path());
        let deleted_comp = self.components.maybe_get(&deleted_component_path);

        if let Some(deleted) = deleted_comp {
            let content = self.gen_struct_definitions_for_component(deleted);
            write_to_file(content, module_path.join("deleted.rs"))?;
            append_to_file(
                format!("pub mod deleted;pub use deleted::{};", deleted.ident()),
                obj_module_path,
            )?;
        }
        Ok(())
    }
}
