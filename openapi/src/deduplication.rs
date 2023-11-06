use heck::ToLowerCamelCase;
use indexmap::map::Entry;
use indexmap::IndexMap;
use tracing::debug;

use crate::components::TypeSpec;
use crate::object_writing::ObjectGenInfo;
use crate::rust_object::{ObjectMetadata, RustObject};
use crate::rust_type::{PathToType, RustType, TypeSource};
use crate::stripe_object::StripeObject;
use crate::types::{ComponentPath, RustIdent};
use crate::visitor::{Visitor, VisitorMut};

#[derive(Debug, Default)]
struct CollectDuplicateObjects {
    objs: IndexMap<RustObject, Vec<ObjectMetadata>>,
}

impl Visitor for CollectDuplicateObjects {
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

#[derive(Debug)]
struct DeduppedObjectInfo {
    ident: RustIdent,
    gen_info: ObjectGenInfo,
}

#[derive(Debug)]
struct DeduplicateObjects<'a> {
    objs: IndexMap<RustObject, DeduppedObjectInfo>,
    component_path: &'a ComponentPath,
}

impl<'a> DeduplicateObjects<'a> {
    pub fn new(path: &'a ComponentPath) -> Self {
        Self { objs: Default::default(), component_path: path }
    }
}

impl<'a> VisitorMut for DeduplicateObjects<'a> {
    fn visit_typ_mut(&mut self, typ: &mut RustType)
    where
        Self: Sized,
    {
        if let Some((obj, _)) = typ.as_object_mut() {
            if let Some(dedup_spec) = self.objs.get(obj) {
                *typ = RustType::Path {
                    path: PathToType::Type {
                        source: TypeSource::Component(self.component_path.clone()),
                        ident: dedup_spec.ident.clone(),
                    },
                    is_ref: false,
                }
            }
        }
        typ.visit_mut(self);
    }
}

fn doc_implied_name(doc: &str) -> Option<&str> {
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

/// Try to infer an identifier given metadata about identical objects
fn infer_ident_for_duplicate_objects(meta: &[ObjectMetadata]) -> Option<RustIdent> {
    let first = meta.first().unwrap();
    if let Some(title) = &first.title {
        // `param` is used very generally and will not be a helpful name to infer
        if title != "param" && meta.iter().all(|m| m.title.as_ref() == Some(title)) {
            return Some(RustIdent::create(title));
        }
    }
    if let Some(field) = &first.field_name {
        if meta.iter().all(|m| m.field_name.as_ref() == Some(field)) {
            return Some(RustIdent::create(field));
        }
    }

    if let Some(desc) = &first.doc {
        if let Some(doc_name) = doc_implied_name(desc) {
            if meta
                .iter()
                .all(|m| m.doc.as_ref().and_then(|m| doc_implied_name(m)) == Some(doc_name))
            {
                return Some(RustIdent::create(doc_name));
            }
        }
    }
    None
}

#[tracing::instrument(level = "debug", skip(comp), fields(path = %comp.path()))]
pub fn deduplicate_types(comp: &mut StripeObject) -> IndexMap<RustIdent, TypeSpec> {
    let mut objs = IndexMap::new();
    let comp_path = comp.path().clone();

    // We run deduplication passes until there are no further changes since one round
    // of deduplicating can enable another
    loop {
        let mut collector = CollectDuplicateObjects::default();
        comp.visit(&mut collector);

        let mut dedupper = DeduplicateObjects::new(&comp_path);
        for (obj, meta) in collector.objs {
            // Nothing to deduplicate
            if meta.len() < 2 {
                continue;
            }
            if let Some(inferred) = infer_ident_for_duplicate_objects(&meta) {
                // Don't add another deduplicated type with the same name as an existing one
                if dedupper.objs.values().all(|o| o.ident != inferred)
                    && !objs.contains_key(&inferred)
                {
                    // Make sure we respect all requirements, e.g. if one item to generate required `Deserialize`,
                    // make sure not to forget that when deduplicating
                    let gen_info = meta.iter().fold(ObjectGenInfo::new(), |prev_meta, meta| {
                        prev_meta.with_shared_requirements(&meta.gen_info)
                    });

                    dedupper.objs.insert(obj, DeduppedObjectInfo { ident: inferred, gen_info });
                }
            }
        }
        // If we weren't able to deduplicate anything new, we're done
        if dedupper.objs.is_empty() {
            break;
        }

        comp.visit_mut(&mut dedupper);
        for (obj, info) in dedupper.objs {
            let mod_path = info.ident.to_lower_camel_case();
            if objs
                .insert(
                    info.ident.clone(),
                    TypeSpec { doc: None, gen_info: info.gen_info, obj, mod_path },
                )
                .is_some()
            {
                panic!("Tried to add duplicate ident {}", info.ident);
            }
        }
    }
    if !objs.is_empty() {
        debug!("Deduplicated {} types", objs.len());
    }
    objs
}
