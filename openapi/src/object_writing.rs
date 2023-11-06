use std::fmt::{Debug, Write};

use crate::components::Components;
use crate::ids::write_object_id;
use crate::printable::Lifetime;
use crate::rust_object::{as_enum_of_objects, ObjectMetadata, RustObject};
use crate::rust_type::RustType;
use crate::stripe_object::{RequestSpec, StripeObject};
use crate::templates::derives::Derives;
use crate::templates::object_trait::{write_object_trait, write_object_trait_for_enum};
use crate::templates::utils::write_doc_comment;
use crate::templates::ObjectWriter;

const ADD_UNKNOWN_VARIANT_THRESHOLD: usize = 12;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ObjectGenInfo {
    pub derives: Derives,
    pub include_constructor: bool,
}

impl ObjectGenInfo {
    pub fn new() -> Self {
        Self { derives: Derives::new(), include_constructor: false }
    }

    pub fn new_deser() -> Self {
        Self::new().deserialize(true).serialize(true)
    }

    pub fn serialize(mut self, serialize: bool) -> Self {
        self.derives.serialize(serialize);
        self
    }

    pub fn deserialize(mut self, deserialize: bool) -> Self {
        self.derives.deserialize(deserialize);
        self
    }

    pub fn include_constructor(mut self) -> Self {
        self.include_constructor = true;
        self
    }

    /// Essentially a union of the requirements
    pub fn with_shared_requirements(&self, other: &Self) -> Self {
        let mut derives = self.derives;
        if other.derives.deserialize {
            derives.deserialize(true);
        }
        if other.derives.serialize {
            derives.serialize(true);
        }
        ObjectGenInfo {
            derives,
            include_constructor: other.include_constructor | self.include_constructor,
        }
    }
}

impl Components {
    fn write_rust_type_objs(&self, typ: &RustType, out: &mut String) {
        let Some((obj, meta)) = typ.as_object() else {
            return;
        };
        self.write_object(obj, meta, out);
    }

    pub fn write_object(&self, obj: &RustObject, metadata: &ObjectMetadata, out: &mut String) {
        let info = metadata.gen_info;

        // If the object contains any references, we'll need to print with a lifetime
        let has_ref = obj.has_reference(self);
        let lifetime = if has_ref { Some(Lifetime::new()) } else { None };
        let ident = &metadata.ident;

        if let Some(doc) = &metadata.doc {
            let comment = write_doc_comment(doc, 0);
            let _ = write!(out, "{comment}");
        }
        let mut writer = ObjectWriter::new(self, ident);
        writer.lifetime(lifetime).derives(info.derives).derives_mut().copy(obj.is_copy(self));
        match obj {
            RustObject::Struct(fields) => {
                let should_derive_default = fields.iter().all(|field| field.rust_type.is_option());
                writer.derives_mut().default(should_derive_default);
                writer.write_struct(out, fields, info.include_constructor);

                for field in fields {
                    if let Some((obj, meta)) = field.rust_type.as_object() {
                        self.write_object(obj, meta, out);
                    }
                }
            }
            RustObject::FieldlessEnum(variants) => {
                let provide_unknown_variant = variants.len() > ADD_UNKNOWN_VARIANT_THRESHOLD
                    && !variants.iter().any(|v| v.variant_name.as_ref() == "Unknown");
                writer.provide_unknown_variant(provide_unknown_variant);
                writer.write_fieldless_enum_variants(out, variants);
            }
            RustObject::Enum(variants) => {
                if let Some(objects) = as_enum_of_objects(self, variants) {
                    writer.write_enum_of_objects(out, self, &objects);
                } else {
                    writer.write_enum_variants(out, variants);
                }
                for variant in variants {
                    if let Some(typ) = &variant.rust_type {
                        if let Some((obj, meta)) = typ.as_object() {
                            self.write_object(obj, meta, out);
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
    let mut out = String::with_capacity(128);

    components.write_object(obj, meta, &mut out);

    let ident = &meta.ident;
    if let Some(id_typ) = comp.id_type() {
        let id_type = components.construct_printable_type(id_typ);
        match &obj {
            RustObject::Struct(_) => {
                write_object_trait(&mut out, ident, &id_type, id_typ.is_option())
            }
            RustObject::Enum(variants) => {
                let Some(object_names) = as_enum_of_objects(components, variants) else {
                    panic!("Object {} is an enum that is not a union of stripe objects", ident);
                };
                write_object_trait_for_enum(components, &mut out, ident, &id_type, &object_names)
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

    for req in specs {
        components.write_rust_type_objs(&req.params, &mut out);

        let req_body = req.gen(components);
        let _ = write!(out, "{}", req_body);

        components.write_rust_type_objs(&req.returned, &mut out);
    }
    out
}
