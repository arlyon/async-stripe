use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter, Write};

use indoc::formatdoc;

use crate::components::{Components, PathInfo};
use crate::ids::{write_object_id, IDS_IN_STRIPE};
use crate::rust_object::{
    FieldedEnumVariant, ObjectMetadata, PrintableFieldedEnumVariant, PrintableStructField,
    RustObject,
};
use crate::rust_type::{CompoundTypeKind, RustType, SimpleType};
use crate::stripe_object::{RequestSpec, StripeObject};
use crate::templates::derives::Derives;
use crate::templates::deserialize::{
    write_deserialize_by_deleted_or_not, write_deserialize_by_object_name,
    write_deserialize_for_struct, DeletedOrNot, NamedObjectVariant,
};
use crate::templates::fielded_enum::write_fielded_enum;
use crate::templates::object_trait::{write_object_trait, write_object_trait_for_enum};
use crate::templates::requests::{PrintablePathParam, PrintableRequestSpec};
use crate::templates::structs::write_struct;
use crate::templates::utils::write_doc_comment;
use crate::types::RustIdent;

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
        let has_borrow = obj.has_borrow();
        let lifetime = if has_borrow { Some(Lifetime::new()) } else { None };
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
                let struct_derives = info
                    .derives
                    .miniserde_deserialize(false)
                    .copy(should_derive_copy)
                    .default(should_derive_default);

                write_struct(
                    out,
                    ident,
                    struct_derives,
                    &printable_fields,
                    info.include_constructor,
                    lifetime,
                );

                if struct_derives.derives_deserialize() {
                    let struct_derive = write_deserialize_for_struct(ident);
                    let _ = writeln!(out, "{struct_derive}");
                }

                for field in fields {
                    if let Some((obj, meta)) = field.rust_type.as_object() {
                        self.write_object(obj, meta, info, out);
                    }
                }
            }
            RustObject::Enum(enum_) => enum_.write_definition_and_methods(out, ident, info.derives),
            RustObject::FieldedEnum(variants) => {
                let printable_variants = variants
                    .iter()
                    .map(|v| {
                        let printable =
                            v.rust_type.as_ref().map(|typ| self.construct_printable_type(typ));
                        PrintableFieldedEnumVariant {
                            rust_type: printable,
                            variant: v.variant.clone(),
                        }
                    })
                    .collect::<Vec<_>>();
                let enum_derives =
                    info.derives.miniserde_deserialize(false).copy(should_derive_copy);
                write_fielded_enum(out, ident, &printable_variants, enum_derives, lifetime);
                if enum_derives.derives_deserialize() {
                    let enum_derive = self
                        .try_write_deserialize_for_enum_with_fields(variants, ident)
                        .expect("Cannot implement `Deserialize`");
                    let _ = writeln!(out, "{enum_derive}");
                }
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

    fn try_write_deserialize_for_enum_with_fields(
        &self,
        variants: &[FieldedEnumVariant],
        ident: &RustIdent,
    ) -> Option<String> {
        let components = variants
            .iter()
            .map(|v| v.rust_type.as_ref().and_then(|t| t.as_reference_path().map(|p| self.get(p))))
            .collect::<Option<Vec<_>>>()?;

        if let Some(as_deleted_or_not) = parse_as_deleted_or_not(&components, variants) {
            Some(write_deserialize_by_deleted_or_not(as_deleted_or_not, ident))
        } else {
            let all_named_objs = components
                .iter()
                .zip(variants)
                .map(|(c, variant)| {
                    c.object_name().map(|obj_name| NamedObjectVariant {
                        object_name: obj_name,
                        variant_name: &variant.variant,
                    })
                })
                .collect::<Option<Vec<_>>>()?;
            let object_names = all_named_objs.iter().map(|o| o.object_name).collect::<HashSet<_>>();

            // Ensure we don't end up with duplicate names - in this case something has gone very wrong
            if object_names.len() != all_named_objs.len() {
                panic!("Enum has duplicate object names");
            }
            Some(write_deserialize_by_object_name(all_named_objs, ident))
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Lifetime(&'static str);

impl Lifetime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_str(self) -> &'static str {
        self.0
    }
}

impl Default for Lifetime {
    fn default() -> Self {
        Self("'a")
    }
}

impl Display for Lifetime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Copy, Clone)]
pub struct PrintableWithLifetime<'a> {
    lifetime: Option<Lifetime>,
    typ: &'a PrintableType,
}

impl<'a> PrintableWithLifetime<'a> {
    pub fn new(typ: &'a PrintableType, lifetime: Option<Lifetime>) -> Self {
        Self { typ, lifetime }
    }
}

#[derive(Debug, Copy, Clone)]
struct PathWithIdent<'a> {
    path: &'a Option<PathInfo>,
    ident: &'a RustIdent,
}

impl<'a> PathWithIdent<'a> {
    fn new(path: &'a Option<PathInfo>, ident: &'a RustIdent) -> Self {
        Self { path, ident }
    }
}

impl<'a> Display for PathWithIdent<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(path) = self.path {
            if let Some(krate) = path.krate {
                write!(f, "{}::", krate.name())?;
            }
            if let Some(path) = &path.path {
                write!(f, "{path}::")?;
            }
        }
        write!(f, "{}", self.ident)
    }
}

impl<'a> Display for PrintableWithLifetime<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Some(lifetime) = self.lifetime else {
            return write!(f, "{}", self.typ);
        };

        match &self.typ {
            PrintableType::QualifiedPath { path, has_borrow, is_borrowed, ident } => {
                let full_path = PathWithIdent::new(path, ident);
                if *is_borrowed {
                    write!(f, "&{lifetime} ")?;
                }
                if *has_borrow {
                    write!(f, "{full_path}<{lifetime}>")
                } else {
                    write!(f, "{full_path}")
                }
            }
            PrintableType::Simple(typ) => {
                if typ.is_borrowed() {
                    write!(f, "&{lifetime} ")?;
                }
                if let Some(import) = typ.import_from() {
                    write!(f, "{import}::")?;
                }
                f.write_str(typ.ident())
            }
            PrintableType::Compound(kind, inner) => {
                let inner = PrintableWithLifetime::new(inner, Some(lifetime));
                match kind {
                    CompoundTypeKind::List => {
                        write!(f, "stripe_types::List<{inner}>")
                    }
                    CompoundTypeKind::Vec => {
                        write!(f, "Vec<{inner}>")
                    }
                    CompoundTypeKind::Slice => {
                        write!(f, "&{lifetime} [{inner}]")
                    }
                    CompoundTypeKind::Expandable => {
                        write!(f, "stripe_types::Expandable<{inner}>")
                    }
                    CompoundTypeKind::Option => {
                        write!(f, "Option<{inner}>")
                    }
                    CompoundTypeKind::Box => {
                        write!(f, "Box<{inner}>")
                    }
                }
            }
        }
    }
}

impl Display for PrintableType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintableType::QualifiedPath { path, is_borrowed, ident, .. } => {
                let full_path = PathWithIdent::new(path, ident);
                if *is_borrowed {
                    f.write_char('&')?;
                }
                write!(f, "{full_path}")
            }
            PrintableType::Simple(typ) => {
                if typ.is_borrowed() {
                    f.write_char('&')?;
                }
                if let Some(import) = typ.import_from() {
                    write!(f, "{import}::")?;
                }
                f.write_str(typ.ident())
            }
            PrintableType::Compound(kind, inner) => match kind {
                CompoundTypeKind::List => {
                    write!(f, "stripe_types::List<{inner}>")
                }
                CompoundTypeKind::Vec => {
                    write!(f, "Vec<{inner}>")
                }
                CompoundTypeKind::Slice => {
                    write!(f, "&[{inner}]")
                }
                CompoundTypeKind::Expandable => {
                    write!(f, "stripe_types::Expandable<{inner}>")
                }
                CompoundTypeKind::Option => {
                    write!(f, "Option<{inner}>")
                }
                CompoundTypeKind::Box => {
                    write!(f, "Box<{inner}>")
                }
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PrintableType {
    QualifiedPath { path: Option<PathInfo>, ident: RustIdent, has_borrow: bool, is_borrowed: bool },
    Simple(SimpleType),
    Compound(CompoundTypeKind, Box<PrintableType>),
}

pub fn write_obj(
    obj: &RustObject,
    meta: &ObjectMetadata,
    comp: &StripeObject,
    components: &Components,
) -> String {
    let gen_info = ObjectGenInfo::new(Derives::deser());
    let mut out = String::with_capacity(128);

    components.write_object(obj, meta, gen_info, &mut out);

    let ident = &meta.ident;
    if let Some(id_typ) = comp.id_type() {
        let id_type = components.construct_printable_type(id_typ);
        match obj {
            RustObject::Struct(_) => write_object_trait(&mut out, ident, &id_type),
            RustObject::FieldedEnum(variants) => {
                write_object_trait_for_enum(&mut out, ident, &id_type, variants)
            }
            RustObject::Enum(_) => {
                panic!("Did not expect enum to have an id");
            }
        }

        if let Some(path) = id_typ.as_id_path() {
            // Only generate the id definition if the paths match - e.g. if we're generating
            // `DeletedAccount`, we don't want to duplicate `AccountId` since `DeletedAccount`
            // uses that same id
            if path == comp.path() && !IDS_IN_STRIPE.contains(path) {
                write_object_id(&mut out, path)
            }
        }
    }
    out
}

pub fn write_requests(
    specs: &[RequestSpec],
    component: &StripeObject,
    components: &Components,
) -> String {
    let mut req_bodies = String::with_capacity(128);

    for req in specs {
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
        printable.gen_code(&mut req_bodies);
    }

    let impl_for_typ = RustType::component_path(component.path().clone());
    let impl_for = components.construct_printable_type(&impl_for_typ);

    let mut out = formatdoc! {
        r#"
        use stripe::{{Client, Response}};
        
        impl {impl_for} {{
            {req_bodies} 
        }}
        "#
    };

    for req in specs {
        components.write_rust_type_objs(
            &req.returned,
            ObjectGenInfo::new(Derives::deser()),
            &mut out,
        );
        if let Some(typ) = &req.params {
            components.write_rust_type_objs(
                typ,
                ObjectGenInfo::new(Derives::new().serialize(true)).include_constructor(),
                &mut out,
            );
        }
    }
    out
}

fn parse_as_deleted_or_not<'a>(
    components: &'a Vec<&'a StripeObject>,
    variants: &'a [FieldedEnumVariant],
) -> Option<DeletedOrNot<'a>> {
    if components.len() != 2 {
        return None;
    }
    let deleted_index = components.iter().position(|c| c.path().is_deleted())?;
    let not_deleted_path = components[deleted_index].path().as_not_deleted();
    let not_deleted_index = components.iter().position(|c| c.path() == &not_deleted_path)?;
    Some(DeletedOrNot {
        deleted_variant: &variants[deleted_index].variant,
        variant: &variants[not_deleted_index].variant,
    })
}
