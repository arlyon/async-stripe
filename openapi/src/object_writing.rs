use std::fmt::Write;

use crate::STRIPE_TYPES;
use crate::components::Components;
use crate::printable::Lifetime;
use crate::rust_object::EnumOfObjects::ObjectUnion;
use crate::rust_object::{ObjectMetadata, ObjectUsage, RustObject, as_enum_of_objects};
use crate::rust_type::RustType;
use crate::stripe_object::{RequestSpec, StripeObject};
use crate::templates::ObjectWriter;
use crate::templates::object_trait::{write_object_trait, write_object_trait_for_enum};
use crate::templates::utils::write_doc_comment;

impl Components {
    fn write_rust_type_objs(&self, typ: &RustType, out: &mut String, usage: ObjectUsage) {
        let Some((obj, meta)) = typ.extract_object() else {
            return;
        };
        self.write_object(obj, meta, usage, out);
    }

    pub fn write_object(
        &self,
        obj: &RustObject,
        metadata: &ObjectMetadata,
        usage: ObjectUsage,
        out: &mut String,
    ) {
        if let Some(doc) = &metadata.doc {
            let comment = write_doc_comment(doc, 0);
            let _ = write!(out, "{comment}");
        }

        // If the object contains any references, we'll need to print with a lifetime
        let has_ref = obj.has_reference(self);
        let lifetime = if has_ref { Some(Lifetime) } else { None };
        let ident = &metadata.ident;

        let mut writer = ObjectWriter::new(self, ident, usage);
        writer.lifetime(lifetime).derive_copy(obj.is_copy(self));

        match obj {
            RustObject::Struct(struct_) => {
                writer.write_struct_definition(out, struct_);
                if usage.used_as_request_param {
                    writer.write_struct_constructor(out, struct_);
                }

                for field in &struct_.fields {
                    if let Some((obj, meta)) = field.rust_type.extract_object() {
                        self.write_object(obj, meta, usage, out);
                    }
                }
            }
            RustObject::FieldlessEnum(variants) => {
                writer.provide_unknown_variant(obj.provide_unknown_variant());
                writer.write_fieldless_enum_variants(out, variants);
            }
            RustObject::Enum(variants) => {
                if let Some(objects) = as_enum_of_objects(self, variants) {
                    writer.write_enum_of_objects(out, &objects);
                } else {
                    writer.write_arbitrary_enum_variants(out, variants);
                }
                for variant in variants {
                    if let Some(typ) = &variant.rust_type {
                        if let Some((obj, meta)) = typ.extract_object() {
                            self.write_object(obj, meta, usage, out);
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

    components.write_object(obj, meta, ObjectUsage::type_def(), &mut out);

    let ident = &meta.ident;
    if let Some(id_typ) = comp.id_type() {
        let id_type = components.construct_printable_type(id_typ);
        match &obj {
            RustObject::Struct(_) => write_object_trait(&mut out, ident, &id_type),
            RustObject::Enum(variants) => {
                let Some(object_names) = as_enum_of_objects(components, variants) else {
                    panic!("Object {ident} is an enum that is not a union of stripe objects");
                };
                let ObjectUnion(objects) = object_names else {
                    panic!("Object {ident} is an enum that is not a union of stripe objects");
                };
                write_object_trait_for_enum(&mut out, ident, &objects)
            }
            RustObject::FieldlessEnum(_) => {
                panic!("Did not expect enum to have an id");
            }
        }

        if let Some(path) = id_typ.as_id_or_opt_id_path() {
            // Only generate the id definition if the paths match - e.g. if we're generating
            // `DeletedAccount`, we don't want to duplicate `AccountId` since `DeletedAccount`
            // uses that same id
            if path == comp.path() {
                let id_ident = comp.id_type_ident();
                let _ = writeln!(out, "{STRIPE_TYPES}::def_id!({id_ident});");
            }
        }
    }
    out
}

pub fn gen_requests(specs: &[RequestSpec], components: &Components) -> String {
    let mut out = String::with_capacity(128);

    for req in specs {
        if let Some(params) = &req.params {
            components.write_rust_type_objs(&params.typ, &mut out, ObjectUsage::request_param());
        }

        let req_body = req.generate(components);
        let _ = write!(out, "{req_body}");

        components.write_rust_type_objs(&req.returned, &mut out, ObjectUsage::return_type());
    }
    out
}
