use std::fmt::Write as _;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};
use indexmap::IndexSet;
use indoc::formatdoc;

use crate::components::{
    build_field_overrides, get_components, Components, Module, ModuleName, Overrides,
};
use crate::crate_inference::Crate;
use crate::ids::{write_object_id, IDS_IN_STRIPE};
use crate::object_context::{write_obj, write_requests, ObjectGenInfo};
use crate::rust_object::ObjectMetadata;
use crate::spec::Spec;
use crate::spec_inference::infer_doc_comment;
use crate::stripe_object::StripeObject;
use crate::templates::cargo_toml::write_crate_toml;
use crate::templates::derives::Derives;
use crate::url_finder::UrlFinder;

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

    fn write_crate(&self, krate: Crate, crate_deps: &IndexSet<Crate>) -> anyhow::Result<()> {
        if krate == Crate::Types {
            if !crate_deps.is_empty() {
                return Err(anyhow!("Types crate should not have dependencies, but has dependencies {crate_deps:#?}!"));
            }
            self.write_generated_for_types_crate()?;
            return Ok(());
        }

        let base_path = PathBuf::from(krate.generated_out_path());

        let toml = write_crate_toml(krate, crate_deps);
        write_to_file(toml, base_path.join("Cargo.toml"))?;

        let crate_name = krate.name();

        // We set up a few things in the base `lib.rs` file:
        // 1. Without this recursion limit increase, `cargo doc` fails
        // 2. The `extern` allows generated code to use absolute paths starting with the crate name instead of `crate`
        // 3. The `Place` declaration supports using `crate::Place` for miniserde deserialization
        let mut mod_rs = formatdoc! {
            r#"
            #![recursion_limit = "256"]
            extern crate self as {crate_name};
            
            #[cfg(feature = "min-ser")]
            miniserde::make_place!(Place);
            "#
        };

        let modules = self
            .components
            .get_crate_members(krate)
            .iter()
            .filter(|m| !self.components.paths_in_types.contains(*m));
        self.write_crate_modules(modules, &base_path.join("src"), &mut mod_rs)?;
        write_to_file(mod_rs, base_path.join("src/lib.rs"))?;

        Ok(())
    }

    fn write_generated_for_types_crate(&self) -> anyhow::Result<()> {
        let base_path = PathBuf::from(Crate::Types.generated_out_path());
        let mut mod_rs_contents = String::new();

        let modules = self.components.get_crate_members(Crate::Types);
        dbg!(&modules);
        self.write_crate_modules(modules, &base_path, &mut mod_rs_contents)?;
        let extra_types = self.components.paths_in_types.iter();
        self.write_crate_modules(extra_types, &base_path, &mut mod_rs_contents)?;

        for (obj, meta) in &self.overrides.overrides {
            let mut out = String::new();
            self.components.write_object(
                obj,
                &ObjectMetadata::new(meta.ident.clone()).doc(meta.doc.clone()),
                ObjectGenInfo::new(Derives::deser()),
                &mut out,
            );
            write_to_file(out, base_path.join(format!("{}.rs", meta.mod_path)))?;
            let _ = writeln!(
                mod_rs_contents,
                "pub mod {0}; pub use {0}::{1};",
                meta.mod_path, meta.ident
            );
        }
        let version = format!(
            "pub const VERSION: crate::ApiVersion = crate::ApiVersion::V{};",
            self.spec.version().replace('-', "_")
        );
        write_to_file(version, base_path.join("version.rs"))?;
        let _ = writeln!(mod_rs_contents, "pub mod version;");

        let mut ids_content = String::new();
        for id_path in IDS_IN_STRIPE.iter() {
            write_object_id(&mut ids_content, id_path);
        }
        write_to_file(ids_content, base_path.join("ids.rs"))?;
        let _ = writeln!(mod_rs_contents, "pub mod ids; pub use ids::*;");
        write_to_file(mod_rs_contents, base_path.join("mod.rs"))?;
        Ok(())
    }

    fn write_crate_modules<'a, I: IntoIterator<Item = &'a ModuleName>>(
        &self,
        modules: I,
        crate_path: &PathBuf,
        mod_rs_contents: &mut String,
    ) -> anyhow::Result<()> {
        for module_name in modules {
            let module = self.components.get_module(module_name);
            match module {
                Module::Package { name, members } => {
                    let _ = write!(mod_rs_contents, "pub mod {name};");
                    let base_path = crate_path.join(name);
                    let mut package_mod_rs = String::new();
                    for inner_path in members {
                        let obj = self.components.get(inner_path);
                        self.write_component(obj, base_path.join(obj.mod_path()))?;
                        let _ = write!(
                            package_mod_rs,
                            "pub mod {0};pub use {0}::{1};",
                            obj.mod_path(),
                            obj.ident()
                        );
                    }
                    write_to_file(package_mod_rs, base_path.join("mod.rs"))?;
                }
                Module::Component { path } => {
                    let obj = self.components.get(path);
                    let mod_path = crate_path.join(obj.mod_path());
                    let _ = write!(
                        mod_rs_contents,
                        "pub mod {0};pub use {0}::{1};",
                        obj.mod_path(),
                        obj.ident()
                    );
                    self.write_component(obj, mod_path)?;
                }
            }
        }
        Ok(())
    }

    pub fn write_files(&self) -> anyhow::Result<()> {
        let crate_graph = self.components.gen_crate_dep_graph();

        for krate in Crate::all() {
            let neighbors = crate_graph.neighbors(*krate);
            self.write_crate(*krate, &neighbors.collect())?;
        }
        Ok(())
    }

    fn write_component(&self, comp: &StripeObject, module_path: PathBuf) -> anyhow::Result<()> {
        let base_obj = comp.rust_obj();
        let doc_url = self.url_finder.url_for_object(comp.path());
        let schema = self.spec.get_component_schema(comp.path());
        let doc_comment = infer_doc_comment(schema, doc_url.as_deref());
        let meta = ObjectMetadata::new(comp.ident().clone()).doc(doc_comment);

        let mut main_content = write_obj(base_obj, &meta, comp, &self.components);
        let has_requests = !comp.requests.is_empty();
        let children: Vec<_> =
            comp.inner_classes().iter().map(|c| self.components.get(c)).collect();

        let deleted_component_path = format!("deleted_{}", comp.path());
        let deleted_comp = self.components.maybe_get(&deleted_component_path);

        // If we have no requests or children, just write a simple file, see e.g. `address.rs`
        // instead of writing a whole `address` module with just the one struct
        if !has_requests && children.is_empty() && deleted_comp.is_none() {
            write_to_file(main_content, format!("{}.rs", module_path.display()))?;
            return Ok(());
        }

        // if has_requests {
        //     let req_content = write_requests(&comp.requests, comp, &self.components);
        //     write_to_file(req_content, module_path.join("requests.rs"))?;
        //     let _ = writeln!(main_content, "pub mod requests;");
        // }

        for component in &children {
            self.write_component(component, module_path.join(component.mod_path()))?;
        }

        if let Some(deleted) = deleted_comp {
            let path = module_path.join("deleted");
            self.write_component(deleted, path)?;
            let _ = writeln!(main_content, "pub mod deleted;pub use deleted::{};", deleted.ident());
        }

        for c in children {
            let _ = write!(main_content, "pub mod {0};pub use {0}::{1};", c.mod_path(), c.ident());
        }
        write_to_file(main_content, module_path.join("mod.rs"))
    }
}

fn write_to_file<C: AsRef<[u8]>, P: AsRef<Path>>(content: C, out_path: P) -> anyhow::Result<()> {
    let mut base = PathBuf::from("out");
    base.push(out_path);
    create_dir_all(base.parent().unwrap())?;
    File::create(&base)
        .with_context(|| format!("Could not create file at {}", base.display()))?
        .write_all(content.as_ref())?;
    Ok(())
}
