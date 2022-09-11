use std::{
    collections::BTreeSet,
    fs::{write, File},
    io::Write,
    iter,
    path::Path,
};

use anyhow::Result;
use itertools::Itertools;

use crate::{file_generator::FileGenerator, metadata::Metadata, url_finder::UrlFinder};

#[derive(Debug)]
pub struct CrateGenerator {
    pub crate_name: String,
    pub objects: Vec<String>,
}

impl CrateGenerator {
    /// Generates this file to the given Path, returning a set
    /// of FileGenerators for the files this one depends on.
    #[tracing::instrument(skip(self, meta, url_finder))]
    pub fn write<'a, T>(
        &mut self,
        base: T,
        meta: &Metadata<'a>,
        url_finder: &UrlFinder,
    ) -> Result<()>
    where
        T: AsRef<Path> + std::fmt::Debug,
    {
        let path = self.crate_name.replace("-", "_");
        // let (out, additional) = self.generate(meta, crate_state, url_finder)?;
        let pathbuf = base.as_ref().join(path);
        let src = pathbuf.join("src/generated");
        std::fs::create_dir_all(&src)?;
        log::info!("writing crate {} to {:?}", self.crate_name, pathbuf);

        println!("objects in crate {}: {:#?}", self.crate_name, self.objects);

        let mut items = Vec::new();
        let mut files = self.get_files();

        for f in files.iter_mut() {
            let x = match f.write(&src, &meta, &self, &url_finder) {
                Ok((a, b, c)) => (a, (b, c)),
                _ => continue,
            };
            items.push(x);
        }

        let (file_names, (shared_objects, imports)): (Vec<_>, (Vec<_>, Vec<_>)) =
            items.into_iter().unzip();

        // let (file_names, (shared_objects, imports)): (Vec<_>, (Vec<_>, Vec<_>)) = self
        //     .get_files()
        //     .into_iter()
        //     .flat_map(|mut f| f.write(&src, &meta, &self, &url_finder))
        //     .map(|(a, b, c)| (a, (b, c)))
        //     .unzip();

        // let (extra_file_names, extra_objects): (Vec<_>, Vec<_>) = shared_objects
        //     .into_iter()
        //     .flatten()
        //     .flat_map(|mut f| f.write(&src, &meta, &self, &url_finder))
        //     .unzip();

        // todo(arlyon): understand the implications of this
        log::warn!("leftover files: {:#?}", shared_objects);
        log::warn!("import: {:#?}", imports);

        let generated = pathbuf.join("src/generated.rs");
        let generated = File::create(generated)?;
        writeln!(
            &generated,
            "{}",
            &file_names.iter().map(|n| format!("pub mod {};", n)).join("\n")
        )?;

        let lib = pathbuf.join("src/lib.rs");
        let lib = File::create(lib)?;
        writeln!(&lib, "pub mod generated;")?;

        writeln!(&lib, "{}", "mod resources {")?;
        writeln!(&lib, "{}", "    pub use {")?;
        writeln!(
            &lib,
            "{}",
            &file_names.iter().map(|n| format!("        crate::generated::{}::*,", n)).join("\n"),
        )?;
        writeln!(&lib, "{}", "    };")?;
        writeln!(&lib, "{}", "}")?;

        let cargo = pathbuf.join("Cargo.toml");

        write(
            &cargo,
            format!(
                r#"[package]
name = "{}"
version = "0.1.0"
edition = "2018"

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
async-stripe-common = {{ path = "../../async_stripe_common" }}
async-stripe-core = {{ path = "../../async_stripe_core" }}
"#,
                &self.crate_name
            ),
        )?;

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub fn get_files(&self) -> Vec<FileGenerator> {
        self.objects
            .iter()
            .filter(|o| !o.starts_with("deleted_"))
            .map(|o| FileGenerator::new(o.to_string()))
            .collect()
    }
}
