use std::{collections::BTreeSet, fs::write, path::Path};

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
    pub fn write<T>(&mut self, base: T, meta: &Metadata, url_finder: &UrlFinder) -> Result<()>
    where
        T: AsRef<Path> + std::fmt::Debug,
    {
        let path = format!("stripe_{}", self.crate_name);
        // let (out, additional) = self.generate(meta, crate_state, url_finder)?;
        let pathbuf = base.as_ref().join(path);
        let src = pathbuf.join("src");
        std::fs::create_dir_all(&src)?;
        log::info!("writing crate {} to {:?}", self.crate_name, pathbuf);

        println!("objects in crate {}: {:#?}", self.crate_name, self.objects);

        let (file_names, shared_objects): (Vec<_>, Vec<_>) = self
            .get_files()
            .into_iter()
            .flat_map(|mut f| f.write(&src, &meta, &self, &url_finder))
            .unzip();

        let (extra_file_names, extra_objects): (Vec<_>, Vec<_>) = shared_objects
            .into_iter()
            .flatten()
            .flat_map(|mut f| f.write(&src, &meta, &self, &url_finder))
            .unzip();

        // todo(arlyon): understand the implications of this
        log::warn!("leftover objects: {:#?}", extra_objects);

        let lib = pathbuf.join("src/lib.rs");

        write(
            &lib,
            file_names
                .into_iter()
                .chain(extra_file_names)
                .map(|n| format!("pub mod {};", n))
                .join("\n"),
        )?;

        let cargo = pathbuf.join("Cargo.toml");

        write(
            &cargo,
            format!(
                "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2018\"\n\n[dependencies]\nserde = {{ version = \"1.0\", features = [\"derive\"] }}\n",
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
