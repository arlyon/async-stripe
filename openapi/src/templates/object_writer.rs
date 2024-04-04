use std::fmt::Write;

use crate::components::Components;
use crate::printable::{Lifetime, PrintableType};
use crate::rust_object::ObjectKind;
use crate::rust_type::RustType;
use crate::templates::derives::Derives;
use crate::types::RustIdent;

#[derive(Copy, Clone)]
pub struct ObjectWriter<'a> {
    pub components: &'a Components,
    pub derives: Derives,
    pub lifetime: Option<Lifetime>,
    pub ident: &'a RustIdent,
    pub provide_unknown_variant: bool,
    pub obj_kind: ObjectKind,
}

impl<'a> ObjectWriter<'a> {
    pub fn new(components: &'a Components, ident: &'a RustIdent, obj_kind: ObjectKind) -> Self {
        Self {
            components,
            derives: Derives::new(),
            lifetime: None,
            ident,
            provide_unknown_variant: false,
            obj_kind,
        }
    }

    pub fn lifetime(&mut self, lifetime: Option<Lifetime>) -> &mut Self {
        self.lifetime = lifetime;
        self
    }

    pub fn provide_unknown_variant(&mut self, unknown_variant: bool) -> &mut Self {
        self.provide_unknown_variant = unknown_variant;
        self
    }

    pub fn derive_copy(&mut self, derive_copy: bool) -> &mut Self {
        self.derives = self.derives.copy(derive_copy);
        self
    }

    pub fn derive_default(&mut self, derive_default: bool) -> &mut Self {
        self.derives = self.derives.default(derive_default);
        self
    }

    pub fn get_printable(&self, typ: &RustType) -> PrintableType {
        self.components.construct_printable_type(typ)
    }

    pub fn lifetime_param(&self) -> &'static str {
        self.lifetime.map(|l| l.as_param()).unwrap_or_default()
    }

    pub fn write_nonexhaustive_attr(&self, out: &mut String) {
        if self.provide_unknown_variant {
            let _ = out.write_str("#[non_exhaustive]");
        }
    }

    pub fn write_automatic_derives(&self, out: &mut String) {
        let mut derives = self.derives;
        derives.serialize = self.obj_kind.should_impl_serialize();
        derives.deserialize = self.obj_kind.should_impl_deserialize();
        write_derives_line(out, derives)
    }
}

pub fn write_derives_line(out: &mut String, derives: Derives) {
    let _ = out.write_str("#[derive(");
    if derives.copy {
        let _ = write!(out, "Copy,");
    }
    let _ = write!(out, "Clone,");
    if derives.debug {
        let _ = write!(out, "Debug,");
    }
    if derives.default {
        let _ = write!(out, "Default,");
    }
    if derives.eq {
        let _ = write!(out, "Eq, PartialEq,");
    }
    if derives.serialize {
        let _ = write!(out, "serde::Serialize,");
    }
    if derives.deserialize {
        let _ = write!(out, "serde::Deserialize,");
    }
    let _ = out.write_str(")]");
}
