use std::fmt::Write as _;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::components::{get_components, Components, Module};
use crate::object_context::{write_obj, write_requests};
use crate::spec::Spec;
use crate::stripe_object::StripeObject;
use crate::url_finder::UrlFinder;

pub struct CodeGen {
    pub components: Components,
}

impl CodeGen {
    pub fn new(spec: Spec, url_finder: UrlFinder) -> anyhow::Result<Self> {
        Ok(Self { components: get_components(&spec, &url_finder)? })
    }

    pub fn write_files(&self) -> anyhow::Result<()> {
        let mut mod_rs = String::new();
        for module in self.components.modules.values() {
            match module {
                Module::Package(package, paths) => {
                    let _ = write!(mod_rs, "pub mod {package};");
                    let base_path = PathBuf::from(package);
                    let mut package_mod_rs = String::new();
                    for inner_path in paths {
                        let component = self.components.get(inner_path);
                        self.write_component(component, base_path.join(component.mod_path()))?;
                        let _ = write!(
                            package_mod_rs,
                            "pub mod {0};pub use {0}::{1};",
                            component.mod_path(),
                            component.ident()
                        );
                    }
                    write_to_file(package_mod_rs, base_path.join("mod"))?;
                }
                Module::Component(path) => {
                    let comp = self.components.get(path);
                    let mod_path = comp.mod_path();
                    let _ = write!(mod_rs, "pub mod {0};pub use {0}::{1};", mod_path, comp.ident());
                    let mod_path = PathBuf::from(mod_path);
                    self.write_component(comp, mod_path)?;
                }
            }
        }

        write_to_file(mod_rs, "mod")?;
        Ok(())
    }

    fn write_component(&self, comp: &StripeObject, module_path: PathBuf) -> anyhow::Result<()> {
        let base_obj = comp.rust_obj();

        let mut main_content = write_obj(base_obj, comp, &self.components);
        let has_requests = !comp.requests.is_empty();
        let children: Vec<_> =
            comp.inner_classes().iter().map(|c| self.components.get(c)).collect();

        let deleted_component_path = format!("deleted_{}", comp.path.as_ref());
        let deleted_comp = self.components.maybe_get(&deleted_component_path);

        // If we have no requests or children, just write a simple file, see e.g. `address.rs`
        // instead of writing a whole `address` module with just the one struct
        if !has_requests && children.is_empty() && deleted_comp.is_none() {
            write_to_file(main_content, module_path)?;
            return Ok(());
        }

        if has_requests {
            let req_content = write_requests(&comp.requests, comp, &self.components);
            write_to_file(req_content, module_path.join("requests"))?;
        }

        for component in &children {
            self.write_component(component, module_path.join(component.mod_path()))?;
        }

        if let Some(deleted) = deleted_comp {
            let path = module_path.join("deleted");
            self.write_component(deleted, path)?;
        }
        if has_requests {
            let _ = writeln!(main_content, "pub mod requests;");
        }
        if let Some(deleted) = deleted_comp {
            let _ = writeln!(main_content, "pub mod deleted;pub use deleted::{};", deleted.ident());
        }

        for c in children {
            let _ = write!(main_content, "pub mod {0};pub use {0}::{1};", c.mod_path(), c.ident());
        }
        write_to_file(main_content, module_path.join("mod"))
    }
}

fn write_to_file<C: AsRef<[u8]>, P: AsRef<Path>>(content: C, out_path: P) -> anyhow::Result<()> {
    let mut base = PathBuf::from("out");
    base.push(out_path);
    base.set_extension("rs");
    create_dir_all(base.parent().unwrap())?;
    File::create(&base)
        .with_context(|| format!("Could not create file at {}", base.display()))?
        .write_all(content.as_ref())?;
    Ok(())
}
