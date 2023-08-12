use std::iter::FromIterator;

use indexmap::map::Entry;
use indexmap::IndexMap;

use crate::rust_object::{ObjectMetadata, RustObject};
use crate::rust_type::{PathToType, RustType};
use crate::types::RustIdent;

#[derive(Debug, Default)]
pub struct Collector {
    objs: IndexMap<RustObject, Vec<ObjectMetadata>>,
}

#[derive(Debug, Default)]
pub struct Dedupper {
    objs: IndexMap<RustObject, RustIdent>,
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

fn infer_ident(meta: Vec<ObjectMetadata>) -> Option<RustIdent> {
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

impl Collector {
    pub fn visit_typ(&mut self, typ: &RustType) {
        match typ {
            RustType::Object(obj, meta) => {
                match self.objs.entry(obj.clone()) {
                    Entry::Occupied(mut occ) => {
                        occ.get_mut().push(meta.clone());
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(vec![meta.clone()]);
                    }
                };
                self.visit_obj(obj);
            }
            RustType::Simple(_) => {}
            RustType::Path { .. } => {}
            RustType::Container(inner) => {
                self.visit_typ(inner.value_typ());
            }
        }
    }

    pub fn visit_obj(&mut self, obj: &RustObject) {
        match obj {
            RustObject::Struct(fields) => {
                for field in fields {
                    self.visit_typ(&field.rust_type);
                }
            }
            RustObject::Enum(variants) => {
                for variant in variants {
                    if let Some(typ) = &variant.rust_type {
                        self.visit_typ(typ);
                    }
                }
            }
            RustObject::FieldlessEnum(_) => {}
        }
    }
}

impl Dedupper {
    pub fn replace_typ(&self, typ: &mut RustType) {
        match typ {
            RustType::Object(obj, _) => {
                let has_ref = obj.has_reference();
                let is_copy = obj.is_copy();
                if let Some(ident) = self.objs.get(obj) {
                    *typ = RustType::Path(PathToType::IntraFile {
                        ident: RustIdent::create(ident),
                        is_ref: false,
                        has_ref,
                        is_copy,
                    });
                } else {
                    self.replace_obj(obj);
                }
            }
            RustType::Simple(_) => {}
            RustType::Path { .. } => {}
            RustType::Container(inner) => {
                self.replace_typ(inner.value_typ_mut());
            }
        }
    }

    pub fn replace_obj(&self, obj: &mut RustObject) {
        match obj {
            RustObject::Struct(fields) => {
                for field in fields {
                    self.replace_typ(&mut field.rust_type);
                }
            }
            RustObject::Enum(variants) => {
                for variant in variants {
                    if let Some(typ) = &mut variant.rust_type {
                        self.replace_typ(typ);
                    }
                }
            }
            RustObject::FieldlessEnum(_) => {}
        }
    }
}

pub fn deduplicate_types(typs: &mut Vec<&mut RustType>) -> IndexMap<RustObject, RustIdent> {
    let mut prev_typs = Vec::from_iter(typs.iter().map(|t| (*t).clone()));
    let mut dedupper = Dedupper::default();
    loop {
        let mut collector = Collector::default();
        for typ in &*typs {
            collector.visit_typ(typ);
        }
        for (obj, meta) in collector.objs {
            if meta.len() <= 1 {
                continue;
            }
            if let Some(inferred) = infer_ident(meta) {
                if dedupper.objs.values().all(|i| i != &inferred) {
                    dedupper.objs.insert(obj, inferred);
                }
            }
        }

        for typ in &mut *typs {
            dedupper.replace_typ(typ);
        }

        if prev_typs.iter().enumerate().all(|(ind, t)| typs[ind] == t) {
            break;
        }
        prev_typs = Vec::from_iter(typs.iter().map(|t| (*t).clone()));
    }
    dedupper.objs
}
