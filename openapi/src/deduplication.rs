use indexmap::map::Entry;
use indexmap::IndexMap;
use tracing::debug;

use crate::rust_object::{ObjectKind, ObjectMetadata, RustObject};
use crate::rust_type::{PathToType, RustType};
use crate::stripe_object::StripeObject;
use crate::types::{ComponentPath, RustIdent};
use crate::visitor::{Visit, VisitMut};

#[derive(Debug, Default)]
struct CollectDuplicateObjects {
    objs: IndexMap<RustObject, Vec<ObjectMetadata>>,
}

impl Visit<'_> for CollectDuplicateObjects {
    fn visit_obj(&mut self, obj: &RustObject, meta: Option<&ObjectMetadata>) {
        if let Some(meta) = meta {
            match self.objs.entry(obj.clone()) {
                Entry::Occupied(mut occ) => {
                    occ.get_mut().push(meta.clone());
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![meta.clone()]);
                }
            }
        };
        obj.visit(self);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DeduppedObjectInfo {
    pub ident: RustIdent,
    pub kind: ObjectKind,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DeduppedObject {
    pub info: DeduppedObjectInfo,
    pub object: RustObject,
}

#[derive(Debug)]
struct DeduplicateObjects {
    objs: IndexMap<RustObject, DeduppedObjectInfo>,
    component_path: ComponentPath,
}

impl DeduplicateObjects {
    pub fn new(path: ComponentPath) -> Self {
        Self { objs: Default::default(), component_path: path }
    }
}

impl VisitMut for DeduplicateObjects {
    fn visit_typ_mut(&mut self, typ: &mut RustType)
    where
        Self: Sized,
    {
        if let Some((obj, _)) = typ.as_object_mut() {
            if let Some(dedup_spec) = self.objs.get(obj) {
                *typ = RustType::Path {
                    path: PathToType::Deduplicated {
                        path: self.component_path.clone(),
                        ident: dedup_spec.ident.clone(),
                    },
                    is_ref: false,
                }
            }
        }
        typ.visit_mut(self);
    }
}

fn implied_name_from_meta_doc(metadata: &ObjectMetadata) -> Option<&str> {
    let doc = metadata.doc.as_ref()?;
    implied_name_from_doc(doc)
}

fn implied_name_from_doc(doc: &str) -> Option<&str> {
    let mut words = doc.split_ascii_whitespace();
    if words.next() != Some("The") {
        return None;
    }
    let second_word = words.next();
    if words.next() != Some("of") {
        return None;
    }
    second_word
}

fn maybe_infer_by_field_name(meta: &[ObjectMetadata]) -> Option<DeduppedObjectInfo> {
    let type_data = meta.iter().find(|m| m.kind == ObjectKind::Type)?;
    let parent = type_data.parent.as_ref()?;
    let field_name = type_data.field_name.as_ref()?;

    if meta.iter().any(|m| m.field_name.as_ref() != Some(field_name)) {
        return None;
    }
    Some(DeduppedObjectInfo {
        ident: RustIdent::joined(parent, field_name),
        kind: ObjectKind::Type,
    })
}

/// Try to infer an identifier given metadata about identical objects
fn infer_dedupped_ident(meta: &[ObjectMetadata]) -> Option<RustIdent> {
    let first = meta.first().unwrap();
    if let Some(title) = &first.title {
        // `param` is used very generally and will not be a helpful name to infer
        if title != "param" && meta.iter().all(|m| m.title.as_ref() == Some(title)) {
            return Some(RustIdent::create(title));
        }
    }

    if let Some(doc_name) = implied_name_from_meta_doc(first) {
        if let Some(parent) = &first.parent {
            if meta.iter().all(|m| {
                implied_name_from_meta_doc(m) == Some(doc_name) && m.parent.as_ref() == Some(parent)
            }) {
                return Some(RustIdent::joined(parent, doc_name));
            }
        }
    }
    None
}

fn infer_dedupped_object_for(
    meta: &[ObjectMetadata],
    obj: &RustObject,
) -> Option<DeduppedObjectInfo> {
    if matches!(obj, RustObject::FieldlessEnum(_)) {
        if let Some(dedupped) = maybe_infer_by_field_name(meta) {
            return Some(dedupped);
        }
    }
    let ident = infer_dedupped_ident(meta)?;
    let first_kind = meta.first().unwrap().kind;
    if meta.iter().all(|m| m.kind == first_kind) {
        return Some(DeduppedObjectInfo { ident, kind: first_kind });
    }
    None
}

#[tracing::instrument(level = "debug", skip(comp), fields(path = %comp.path()))]
pub fn deduplicate_types(comp: &mut StripeObject) -> IndexMap<RustIdent, DeduppedObject> {
    let mut objs = IndexMap::new();
    let comp_path = comp.path().clone();

    // We run deduplication passes until there are no further changes since one round
    // of deduplicating can enable another
    loop {
        let mut collector = CollectDuplicateObjects::default();
        comp.visit(&mut collector);

        let mut dedupper = DeduplicateObjects::new(comp_path.clone());
        for (obj, meta) in collector.objs {
            // Nothing to deduplicate
            if meta.len() < 2 {
                continue;
            }
            if let Some(inferred) = infer_dedupped_object_for(&meta, &obj) {
                let ident = &inferred.ident;
                // Don't add another deduplicated type with the same name as an existing one
                if dedupper.objs.values().all(|o| &o.ident != ident) && !objs.contains_key(ident) {
                    dedupper.objs.insert(obj, inferred);
                }
            }
        }
        // If we weren't able to deduplicate anything new, we're done
        if dedupper.objs.is_empty() {
            break;
        }

        comp.visit_mut(&mut dedupper);
        for (obj, info) in dedupper.objs {
            let ident = info.ident.clone();
            if let Some(dup) = objs.insert(ident, DeduppedObject { info, object: obj }) {
                panic!("Tried to add duplicate ident {}", dup.info.ident);
            }
        }
    }
    if !objs.is_empty() {
        debug!("Deduplicated {} types", objs.len());
    }
    objs
}
