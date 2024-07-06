use anyhow::Context;
use indexmap::IndexMap;

use crate::components::{Components, RequestSource};
use crate::rust_object::{ObjectMetadata, ObjectUsage, RustObject};
use crate::rust_type::{PathToType, RustType};
use crate::stripe_object::OperationType;
use crate::types::RustIdent;
use crate::visitor::VisitMut;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OverrideMetadata {
    pub metadata: ObjectMetadata,
    pub mod_path: String,
}

pub struct Overrides {
    pub overrides: IndexMap<RustObject, OverrideMetadata>,
}

impl Overrides {
    pub fn new(components: &Components) -> anyhow::Result<Self> {
        let mut overrides = IndexMap::new();
        for override_ in OVERRIDES {
            let (obj, meta) = get_override_object(components, override_).with_context(|| {
                format!("Failed to construct override for source {override_:?}")
            })?;
            overrides.insert(obj, meta);
        }
        Ok(Self { overrides })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct OverrideData {
    pub doc: &'static str,
    pub mod_path: &'static str,
    pub ident: &'static str,
    pub source: RequestSource,
}

const OVERRIDES: &[OverrideData] = &[OverrideData {
    doc: "",
    mod_path: "api_version",
    ident: "ApiVersion",
    source: RequestSource {
        path: "/webhook_endpoints",
        op: OperationType::Post,
        field_name: "api_version",
    },
}];

fn get_override_object(
    components: &Components,
    data: &OverrideData,
) -> anyhow::Result<(RustObject, OverrideMetadata)> {
    let req = components.get_request_spec(data.source).context("Request source not found")?;
    let (obj, _) = req
        .get_param_field(data.source.field_name)
        .context("Could not extract field")?
        .extract_object()
        .context("Not an object")?;
    let metadata = ObjectMetadata::new(RustIdent::unchanged(data.ident)).doc(data.doc.to_string());

    Ok((obj.clone(), OverrideMetadata { metadata, mod_path: data.mod_path.to_string() }))
}

impl VisitMut for Overrides {
    fn visit_typ_mut(&mut self, typ: &mut RustType, usage: ObjectUsage) {
        if let Some((obj, _)) = typ.as_object_mut() {
            if let Some(meta) = self.overrides.get(obj) {
                *typ = RustType::path(PathToType::Shared(meta.metadata.ident.clone()), false);
            }
        }
        typ.visit_mut(self, usage);
    }
}
