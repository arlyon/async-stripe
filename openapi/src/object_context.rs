use std::fmt::{Debug, Write};

use crate::components::Components;
use crate::dedup::deduplicate_types;
use crate::ids::write_object_id;
use crate::printable::{Lifetime, PrintableEnumVariant, PrintableStructField};
use crate::rust_object::{ObjectMetadata, RustObject};
use crate::rust_type::RustType;
use crate::stripe_object::{RequestSpec, StripeObject};
use crate::templates::derives::Derives;
use crate::templates::enums::{write_enum_variants, write_fieldless_enum_variants};
use crate::templates::object_trait::{write_object_trait, write_object_trait_for_enum};
use crate::templates::requests::{PrintablePathParam, PrintableRequestSpec};
use crate::templates::structs::write_struct;
use crate::templates::utils::write_doc_comment;

#[derive(Copy, Clone, Debug)]
pub struct ObjectGenInfo {
    pub derives: Derives,
    pub include_constructor: bool,
}

impl ObjectGenInfo {
    pub fn new(derives: Derives) -> Self {
        Self { derives, include_constructor: false }
    }

    pub fn include_constructor(mut self) -> Self {
        self.include_constructor = true;
        self
    }
}

impl Components {
    fn write_rust_type_objs(&self, typ: &RustType, info: ObjectGenInfo, out: &mut String) {
        let Some((obj, meta)) = typ.as_object() else {
            return;
        };
        self.write_object(obj, meta, info, out);
    }

    pub fn write_object(
        &self,
        obj: &RustObject,
        metadata: &ObjectMetadata,
        info: ObjectGenInfo,
        out: &mut String,
    ) {
        // If the object contains any references, we'll need to print with a lifetime
        let has_ref = obj.has_reference();
        let lifetime = if has_ref { Some(Lifetime::new()) } else { None };
        let should_derive_copy = obj.is_copy();
        let ident = &metadata.ident;

        if let Some(doc) = &metadata.doc {
            let comment = write_doc_comment(doc, 0);
            let _ = write!(out, "{comment}");
        }
        match obj {
            RustObject::Struct(fields) => {
                let should_derive_default = fields.iter().all(|field| field.rust_type.is_option());
                let printable_fields = fields
                    .iter()
                    .map(|f| {
                        let printable = self.construct_printable_type(&f.rust_type);
                        PrintableStructField::from_field(f, printable)
                    })
                    .collect::<Vec<_>>();
                let struct_derives =
                    info.derives.copy(should_derive_copy).default(should_derive_default);

                write_struct(
                    out,
                    ident,
                    struct_derives,
                    &printable_fields,
                    info.include_constructor,
                    lifetime,
                );

                for field in fields {
                    if let Some((obj, meta)) = field.rust_type.as_object() {
                        self.write_object(obj, meta, info, out);
                    }
                }
            }
            RustObject::FieldlessEnum(variants) => {
                write_fieldless_enum_variants(out, variants, ident, info.derives)
            }
            RustObject::Enum(variants) => {
                let printable_variants = variants
                    .iter()
                    .map(|v| {
                        let printable =
                            v.rust_type.as_ref().map(|typ| self.construct_printable_type(typ));
                        PrintableEnumVariant { rust_type: printable, variant: v.variant.clone() }
                    })
                    .collect::<Vec<_>>();
                let enum_derives = info.derives.copy(should_derive_copy);
                write_enum_variants(out, ident, &printable_variants, enum_derives, lifetime);
                for variant in variants {
                    if let Some(typ) = &variant.rust_type {
                        if let Some((obj, meta)) = typ.as_object() {
                            self.write_object(obj, meta, info, out);
                        }
                    }
                }
            }
        }
    }
}

pub fn gen_obj(
    obj: &RustObject,
    meta: &ObjectMetadata,
    comp: &StripeObject,
    components: &Components,
) -> String {
    let gen_info = ObjectGenInfo::new(Derives::new_deser());
    let mut out = String::with_capacity(128);

    let mut obj = obj.clone();

    // NB: we deduplicate the fields / variants of a top-level struct, not the object
    // itself
    let mut typs = obj.typs_mut();
    let dedupped_objs = deduplicate_types(&mut typs);
    components.write_object(&obj, meta, gen_info, &mut out);

    for (obj, ident) in dedupped_objs {
        let typ = RustType::Object(obj, ObjectMetadata::new(ident));
        components.write_rust_type_objs(&typ, gen_info, &mut out);
    }

    let ident = &meta.ident;
    if let Some(id_typ) = comp.id_type() {
        let id_type = components.construct_printable_type(id_typ);
        match &obj {
            RustObject::Struct(_) => write_object_trait(&mut out, ident, &id_type),
            RustObject::Enum(variants) => {
                write_object_trait_for_enum(&mut out, ident, &id_type, variants)
            }
            RustObject::FieldlessEnum(_) => {
                panic!("Did not expect enum to have an id");
            }
        }

        if let Some(path) = id_typ.as_id_path() {
            // Only generate the id definition if the paths match - e.g. if we're generating
            // `DeletedAccount`, we don't want to duplicate `AccountId` since `DeletedAccount`
            // uses that same id
            if path == comp.path() {
                write_object_id(&mut out, path)
            }
        }
    }
    out
}

pub fn gen_requests(specs: &[RequestSpec], components: &Components) -> String {
    let mut out = String::with_capacity(128);
    let mut specs = Vec::from(specs);
    let mut req_typs = vec![];
    for spec in &mut specs {
        if let Some(param_typ) = &mut spec.params {
            req_typs.push(param_typ);
        }
    }

    let dedupped_objs = deduplicate_types(&mut req_typs);

    for req in &specs {
        let printable = PrintableRequestSpec {
            path_params: req
                .path_params
                .iter()
                .map(|p| PrintablePathParam {
                    typ: components.construct_printable_type(&p.rust_type),
                    name: &p.name,
                })
                .collect(),
            doc_comment: req.doc_comment.as_deref(),
            method_name: &req.method_name,
            request_path: &req.req_path,
            param_type: req.params.as_ref().map(|typ| components.construct_printable_type(typ)),
            returned: components.construct_printable_type(&req.returned),
            method_type: req.method_type,
        };
        printable.gen_code(&mut out);
    }

    let params_gen_info = ObjectGenInfo::new(Derives::new().serialize(true)).include_constructor();

    for req in &specs {
        components.write_rust_type_objs(
            &req.returned,
            ObjectGenInfo::new(Derives::new_deser()),
            &mut out,
        );
        if let Some(typ) = &req.params {
            components.write_rust_type_objs(typ, params_gen_info, &mut out);
        }
    }
    for (obj, ident) in dedupped_objs {
        let typ = RustType::Object(obj, ObjectMetadata::new(ident));
        components.write_rust_type_objs(&typ, params_gen_info, &mut out);
    }
    out
}
