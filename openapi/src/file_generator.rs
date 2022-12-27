use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet},
    fs::write,
    path::Path,
};

use anyhow::{Context, Result};
use heck::SnakeCase;

use crate::codegen::{
    gen_enums, gen_impl_requests, gen_object_trait, gen_objects, gen_prelude, gen_unions,
};
use crate::spec::{as_first_enum_value, as_object_properties};
use crate::{
    codegen::gen_emitted_structs, codegen::gen_generated_schemas, codegen::gen_inferred_params,
    codegen::gen_struct, metadata::Metadata, types::InferredEnum, types::InferredObject,
    types::InferredParams, types::InferredStruct, types::InferredUnion, url_finder::UrlFinder,
};

#[derive(Default, Debug)]
pub struct FileGenerator {
    pub name: String,
    /// The ids that must be imported in this file.
    pub use_ids: BTreeSet<String>,
    /// The config that must be imported in this file.
    pub use_config: BTreeSet<&'static str>,
    /// The params that must be imported in this file.
    pub use_params: BTreeSet<&'static str>,
    /// The resources that must be imported in this file.
    pub use_resources: BTreeSet<String>,
    /// Extra (simple) enums that were / will be generated in this file.
    pub inferred_enums: BTreeMap<String, InferredEnum>,
    /// Extra (complex) enums that were / will be generated in this file.
    pub inferred_unions: BTreeMap<String, InferredUnion>,
    /// Extra structs that were / will be generated in this file.
    pub inferred_structs: BTreeMap<String, InferredStruct>,
    /// The request parameter structs that were / will be generated in this file.
    pub inferred_parameters: BTreeMap<String, InferredParams>,
    /// The schemas that were / will be generated in this file.
    pub generated_schemas: BTreeMap<String, bool>,
    /// New experimental struct thatclear
    ///  will eventually do most of the general work
    pub generated_objects: BTreeMap<String, InferredObject>,
}

impl FileGenerator {
    pub fn new(object_name: String) -> Self {
        Self { name: object_name, ..Default::default() }
    }

    fn get_path(&self) -> String {
        self.name.replace('.', "_").to_snake_case() + ".rs"
    }

    /// Generates this file to the given Path, returning a set
    /// of FileGenerators for the files this one depends on.
    #[tracing::instrument(skip(self, meta, url_finder))]
    pub fn write<T>(
        &mut self,
        base: T,
        meta: &Metadata,
        url_finder: &UrlFinder,
    ) -> Result<BTreeSet<FileGenerator>>
    where
        T: AsRef<Path> + std::fmt::Debug,
    {
        let path = self.get_path();
        let (out, additional) = self.generate(meta, url_finder)?;
        let pathbuf = base.as_ref().join(path);
        log::debug!("writing object {} to {:?}", self.name, pathbuf);
        write(&pathbuf, out.as_bytes())?;
        Ok(additional)
    }

    /// Generates this file, returning a set of FileGenerators
    /// for the files this one depends on.
    #[tracing::instrument(skip(self, meta, url_finder))]
    pub fn generate(
        &mut self,
        meta: &Metadata,
        url_finder: &UrlFinder,
    ) -> Result<(String, BTreeSet<FileGenerator>)> {
        let mut shared_objects = BTreeSet::<FileGenerator>::new();

        let properties = meta
            .spec
            .component_schemas()
            .get(&self.name)
            .and_then(|s| s.as_item())
            .and_then(as_object_properties)
            .context("No properties")?;

        let struct_ = gen_struct(self, meta, &mut shared_objects, url_finder);
        let (object_trait, (request_impls, paginable)) = if let Some(object_literal) =
            properties.get("object").and_then(|o| o.as_item()).and_then(|s| as_first_enum_value(s))
        {
            (
                gen_object_trait(self, meta, &object_literal),
                gen_impl_requests(self, meta).unwrap_or_default(),
            )
        } else {
            Default::default()
        };

        let schemas = gen_generated_schemas(self, meta, &mut shared_objects);
        let inferred_params = gen_inferred_params(self, meta, &mut shared_objects);
        let emitted_structs = gen_emitted_structs(self, meta, &mut shared_objects);
        let unions = gen_unions(&self.inferred_unions, meta);
        let enums = gen_enums(&self.inferred_enums, meta);
        let objects = gen_objects(&self.generated_objects);
        let prelude = gen_prelude(self, meta);

        let written = vec![
            prelude,
            struct_,
            request_impls,
            object_trait,
            schemas,
            inferred_params,
            paginable,
            emitted_structs,
            unions,
            enums,
            objects,
        ]
        .join("\n");

        Ok((written, shared_objects))
    }

    pub fn insert_enum(&mut self, name: impl Into<String>, enum_: InferredEnum) {
        if let Err(other) = self.try_insert_enum(name, enum_.clone()) {
            panic!("conflicting enums are not compatible:\n\t{:?}\n\t!=\n\t{:?}", enum_, other);
        }
    }

    pub fn try_insert_enum(
        &mut self,
        name: impl Into<String>,
        enum_: InferredEnum,
    ) -> Result<(), &InferredEnum> {
        let name = name.into();
        let mut enum_ = enum_;
        enum_.options.sort();
        if let Entry::Vacant(e) = self.inferred_enums.entry(name.clone()) {
            e.insert(enum_);
            return Ok(());
        }
        if let Some(other) = self.inferred_enums.get(&name) {
            if enum_.options != other.options {
                return Err(other);
            }
        }
        Ok(())
    }

    pub fn insert_struct(&mut self, name: impl Into<String>, struct_: InferredStruct) {
        if let Err(other) = self.try_insert_struct(name, struct_.clone()) {
            panic!("conflicting structs are not compatible:\n\t{:?}\n\t!=\n\t{:?}", struct_, other);
        }
    }

    pub fn try_insert_struct(
        &mut self,
        name: impl Into<String>,
        struct_: InferredStruct,
    ) -> Result<(), &InferredStruct> {
        let name = name.into();
        if let Entry::Vacant(e) = self.inferred_structs.entry(name) {
            e.insert(struct_);
            return Ok(());
        }
        Ok(())
    }

    pub fn add_use(&mut self, use_path: &str) {
        for path in use_path.split(',') {
            match path {
                "" | "String" => {}
                "Metadata" => {
                    self.use_params.insert("Metadata");
                }
                "Expandable" => {
                    self.use_params.insert("Expandable");
                }
                path if path.ends_with("Id") && path != "TaxId" => {
                    self.use_ids.insert(path.into());
                }
                path => {
                    self.use_resources.insert(path.into());
                }
            }
        }
    }
}

impl PartialEq for FileGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for FileGenerator {}

impl PartialOrd for FileGenerator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for FileGenerator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
