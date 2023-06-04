use std::borrow::Borrow;
use std::fmt::Write;

use anyhow::anyhow;
use heck::SnakeCase;
use openapiv3::{
    AdditionalProperties, IntegerFormat, ObjectType, ReferenceOr, Schema, SchemaKind, StringType,
    Type, VariantOrUnknownOrEmpty,
};

use crate::rust_object::{
    EnumVariant, FieldedEnumVariant, RustEnum, RustObjectBuilder, RustObjectData, StructField,
};
use crate::rust_type::{CompoundTypeKind, ExtType, IntType, RustType, SimpleType};
use crate::spec::{
    as_data_array_item, as_object_enum_name, is_enum_with_just_empty_string, ExpansionResources,
};
use crate::types::{ComponentPath, RustIdent};

#[derive(Copy, Clone, Debug)]
pub struct Inference<'a> {
    can_borrow: bool,
    id_path: Option<&'a ComponentPath>,
    curr_ident: &'a RustIdent,
    field_name: Option<&'a str>,
    description: Option<&'a str>,
    required: bool,
}

impl<'a> Inference<'a> {
    pub fn new(ident: &'a RustIdent) -> Self {
        Self {
            can_borrow: false,
            field_name: None,
            description: None,
            required: false,
            curr_ident: ident,
            id_path: None,
        }
    }

    pub fn can_borrow(mut self) -> Self {
        self.can_borrow = true;
        self
    }

    pub fn field_name(mut self, name: &'a str) -> Self {
        self.field_name = Some(name);
        self
    }

    pub fn description(mut self, desc: &'a str) -> Self {
        self.description = Some(desc);
        self
    }

    pub fn maybe_description(mut self, desc: Option<&'a str>) -> Self {
        self.description = desc;
        self
    }

    pub fn id_path(mut self, path: &'a ComponentPath) -> Self {
        self.id_path = Some(path);
        self
    }

    pub fn curr_ident(mut self, ident: &'a RustIdent) -> Self {
        self.curr_ident = ident;
        self
    }

    pub fn required(mut self, is_required: bool) -> Self {
        self.required = is_required;
        self
    }

    fn next_ident(&self) -> RustIdent {
        if let Some(field_name) = self.field_name {
            RustIdent::joined(self.curr_ident, field_name)
        } else {
            self.curr_ident.clone()
        }
    }

    fn build_object_type(self, data: RustObjectData, ident: RustIdent) -> RustType {
        RustType::Object(RustObjectBuilder::new().maybe_doc(self.description).build(data, ident))
    }

    pub fn infer_schema_type(&self, field: &Schema) -> RustType {
        let base_type = self.infer_base_type(field);
        let is_nullable = field.schema_data.nullable;
        if !self.required || is_nullable {
            base_type.into_nullable()
        } else {
            base_type
        }
    }

    fn should_infer_as_id_type(&self) -> bool {
        let Some(desc) = self.description else {
            return false;
        };
        if desc.starts_with("A cursor for use in pagination")
            || desc.starts_with("An object ID cursor")
        {
            return true;
        }
        if self.field_name == Some("id")
            && (desc.starts_with("Unique identifier")
                || desc.starts_with("The ID of the")
                || desc.starts_with("The identifier for the")
                || desc.starts_with("The [ID of the"))
        {
            return true;
        }

        false
    }

    fn infer_base_type(&self, field: &Schema) -> RustType {
        if self.field_name == Some("metadata") {
            return RustType::Simple(SimpleType::Ext(ExtType::Metadata {
                borrowed: self.can_borrow,
            }));
        }

        match &field.schema_kind {
            SchemaKind::Type(Type::Boolean {}) => RustType::bool(),
            SchemaKind::Type(Type::Number(_)) => RustType::float(),
            SchemaKind::Type(Type::Integer(format)) => self.infer_integer_type(&format.format),
            SchemaKind::Type(Type::String(typ)) => self.infer_string_typ(typ),
            SchemaKind::Type(Type::Array(typ)) => {
                let element = typ.items.as_ref().expect("Array type had no items field");
                let element_type = self.required(true).infer_schema_or_ref_type(element);
                if self.can_borrow {
                    RustType::slice(element_type)
                } else {
                    RustType::vec(element_type)
                }
            }
            SchemaKind::Type(Type::Object(typ)) => self.infer_object_typ(typ, field),
            SchemaKind::AnyOf { any_of: fields } | SchemaKind::OneOf { one_of: fields } => {
                self.infer_any_or_one_of(fields, field)
            }
            _ => {
                panic!("unhandled field type");
            }
        }
    }

    fn infer_string_typ(&self, typ: &StringType) -> RustType {
        let variants = build_enum_variants(&typ.enumeration);
        if !variants.is_empty() {
            let mut enum_obj = RustEnum::new();
            for variant in variants {
                enum_obj.add_variant(variant);
            }
            return self.build_object_type(RustObjectData::Enum(enum_obj), self.next_ident());
        }
        if let Some(f_name) = self.field_name {
            if f_name == "currency" || f_name.ends_with("_currency") {
                return RustType::ext(ExtType::Currency);
            }
        }
        // FIXME: this is a hack to ensure the id's used for pagination are
        // owned. This can be removed with cleverer codegen, but likely
        // best to wait until issues like https://github.com/arlyon/async-stripe/issues/246
        // resolved
        let for_pagination =
            self.field_name == Some("ending_before") || self.field_name == Some("starting_after");
        if for_pagination {
            return RustType::string();
        }

        if !for_pagination && self.should_infer_as_id_type() {
            if let Some(id_path) = self.id_path {
                return RustType::ObjectId { path: id_path.clone(), borrowed: self.can_borrow };
            }
        }

        if self.can_borrow {
            RustType::Simple(SimpleType::Str)
        } else {
            RustType::string()
        }
    }

    fn infer_object_typ(&self, typ: &ObjectType, field: &Schema) -> RustType {
        if as_object_enum_name(field).as_deref() == Some("list") {
            let element = as_data_array_item(typ).unwrap_or_else(|| {
                panic!("Expected to find array item but found {:?}", field.schema_kind)
            });
            let element_type = self.infer_schema_or_ref_type(element);
            return RustType::list(element_type);
        }

        if let Some(AdditionalProperties::Schema(additional)) = &typ.additional_properties {
            return self.infer_schema_or_ref_type(additional);
        }
        // Generate the struct type
        let mut fields = vec![];
        let next_ident = self.next_ident();
        for (prop_field_name, field_spec) in &typ.properties {
            let is_required = typ.required.contains(prop_field_name);
            let field_desc =
                field_spec.as_item().and_then(|i| i.schema_data.description.as_deref());
            fields.push(
                self.required(is_required)
                    .curr_ident(&next_ident)
                    .maybe_description(field_desc)
                    .build_struct_field(prop_field_name, field_spec),
            );
        }
        self.build_object_type(RustObjectData::Struct(fields), next_ident)
    }

    fn infer_any_or_one_of(&self, fields: &[ReferenceOr<Schema>], field: &Schema) -> RustType {
        let fields = fields
            .iter()
            .filter(|field| !is_enum_with_just_empty_string(field))
            .collect::<Vec<_>>();
        if fields.len() == 1 {
            self.infer_schema_or_ref_type(fields[0])
        } else if let Some(resources) = field.schema_data.extensions.get("x-expansionResources") {
            parse_expansion_resources(resources).expect("Failed to parse expansion resources")
        } else if fields[0].as_item().and_then(|s| s.schema_data.title.as_deref())
            == Some("range_query_specs")
        {
            RustType::ext(ExtType::RangeQueryTs)
        } else {
            let enum_ = self.build_fielded_enum(fields).expect("Could not build enum with fields");
            self.build_object_type(RustObjectData::FieldedEnum(enum_), self.next_ident())
        }
    }

    pub fn infer_schema_or_ref_type<T: Borrow<Schema>>(&self, field: &ReferenceOr<T>) -> RustType {
        let typ = match field {
            ReferenceOr::Reference { reference } => {
                let mut typ = RustType::from_ref(reference);
                if !self.required {
                    typ = typ.into_nullable();
                }
                typ
            }
            ReferenceOr::Item(schema) => {
                let schema = schema.borrow();
                self.infer_schema_type(schema)
            }
        };
        if typ.should_box() {
            RustType::boxed(typ)
        } else {
            typ
        }
    }

    pub fn build_struct_field<T: Borrow<Schema>>(
        &self,
        field_name: &str,
        field: &ReferenceOr<T>,
    ) -> StructField {
        let rust_type = self.field_name(field_name).infer_schema_or_ref_type(field);
        let mut field_rename = field_name.to_snake_case();
        if field_rename == "type" {
            field_rename = "type_".into();
        }
        let mut struct_field = StructField::new(&field_rename, rust_type, self.required);
        if let Some(doc) = self.description {
            struct_field = struct_field.doc(doc);
        }
        if field_rename != field_name {
            struct_field = struct_field.rename_as(field_name);
        }
        struct_field
    }

    fn build_fielded_enum(
        &self,
        fields: Vec<&ReferenceOr<Schema>>,
    ) -> anyhow::Result<Vec<FieldedEnumVariant>> {
        let mut variants = Vec::with_capacity(fields.len());
        for option in fields {
            match option.borrow() {
                ReferenceOr::Reference { reference } => {
                    let schema_path = ComponentPath::from_reference(reference);
                    variants.push(FieldedEnumVariant::new(
                        RustIdent::create(&schema_path),
                        RustType::from_ref(reference),
                    ));
                }
                ReferenceOr::Item(item) => {
                    let mut ctx = self.required(true);
                    let inferred_name = infer_enum_variant_name(item);
                    if let Some(name) = inferred_name {
                        ctx = ctx.field_name(name);
                    }
                    let rust_type = ctx.required(true).infer_schema_type(item);
                    if let Some(RustObjectData::Enum(enum_)) = rust_type.as_rust_obj_data() {
                        for variant in &enum_.variants {
                            variants
                                .push(FieldedEnumVariant::fieldless(variant.variant_name.clone()));
                        }
                    } else {
                        let variant_ident = if let Some(name) = inferred_name {
                            RustIdent::create(name)
                        } else if let RustType::Simple(typ) = &rust_type {
                            RustIdent::create(typ.ident())
                        } else {
                            return Err(anyhow!(
                                "Could not infer a variant name for {item:?} and type {:?}",
                                rust_type
                            ));
                        };
                        variants.push(FieldedEnumVariant::new(variant_ident, rust_type));
                    }
                }
            }
        }
        Ok(variants)
    }

    fn infer_integer_type(&self, int_fmt: &VariantOrUnknownOrEmpty<IntegerFormat>) -> RustType {
        if matches!(int_fmt, VariantOrUnknownOrEmpty::Unknown(inner) if inner == "unix-time") {
            return RustType::ext(ExtType::Timestamp);
        }

        // Infer based on field name, otherwise default
        let Some(name) = self.field_name else { return RustType::int(IntType::I64)};

        let name_snake = name.to_snake_case();
        let name_words = name_snake.split('_').collect::<Vec<_>>();
        if name_words.contains(&"date") {
            RustType::ext(ExtType::Timestamp)
        } else if name == "monthly_anchor" {
            RustType::int(IntType::U8)
        } else if name_words.contains(&"days") {
            RustType::int(IntType::U32)
        } else if name_words.contains(&"count")
            || name_words.contains(&"size")
            || name_words.contains(&"quantity")
        {
            RustType::int(IntType::U64)
        } else {
            RustType::int(IntType::I64)
        }
    }
}

fn parse_expansion_resources(resources: &serde_json::Value) -> anyhow::Result<RustType> {
    let expansion_resources = serde_json::from_value::<ExpansionResources>(resources.clone())?;

    let schema_kind = expansion_resources.into_schema_kind();
    let SchemaKind::OneOf {one_of} = schema_kind else {
        return Err(anyhow!("Expected expansion resources to only contain `oneOf`"));
    };
    if one_of.len() != 1 {
        return Err(anyhow!("Expected a single specification in expansion resources"));
    }
    let ReferenceOr::Reference {reference} = one_of.first().unwrap() else {
        return Err(anyhow!("Expected expansion resource to only contain a schema reference"));
    };
    Ok(RustType::Compound(CompoundTypeKind::Expandable, Box::new(RustType::from_ref(reference))))
}

fn build_enum_variants(options: &[Option<String>]) -> Vec<EnumVariant> {
    let mut enum_variants = vec![];
    for wire_name in options.iter().flatten() {
        if wire_name.trim().is_empty() {
            continue;
        }
        let variant_name = match wire_name.as_str() {
            "*" => RustIdent::create("all"),
            n => {
                if n.chars().next().unwrap().is_ascii_digit() {
                    RustIdent::from_valid(format!("V{}", n.to_string().replace(['-', '.'], "_")))
                } else {
                    // Hit for the case of timezones, e.g. Etc+7 and Etc-7
                    if wire_name.contains(['+', '-']) {
                        let cleaned = wire_name.replace('+', "Plus").replace('-', "Minus");
                        RustIdent::create(&cleaned)
                    } else {
                        RustIdent::create(wire_name)
                    }
                }
            }
        };
        enum_variants.push(EnumVariant { wire_name: wire_name.clone(), variant_name });
    }
    enum_variants
}

fn infer_enum_variant_name(schema: &Schema) -> Option<&str> {
    if let Some(title) = &schema.schema_data.title {
        return Some(title);
    }
    if let Some(desc) = &schema.schema_data.description {
        if desc.contains("The ID of") {
            return Some("Id");
        }
    }
    None
}

/// Try to generate the best doc comment we can using the information provided. `doc_url`
/// is the source for more info, currently provided by `UrlFinder`.
pub fn infer_doc_comment(schema: &Schema, doc_url: Option<&str>) -> String {
    let mut doc_comment = if let Some(descr) = &schema.schema_data.description {
        descr.clone()
    } else if let Some(title) = &schema.schema_data.title {
        format!("The resource representing a Stripe {title}")
    } else {
        // This case is rare, but possible
        String::new()
    };
    if let Some(doc) = doc_url {
        let _ = writeln!(doc_comment, "\n\nFor more details see <{doc}>");
    }
    doc_comment
}

pub fn infer_id_name(obj_name: &str) -> RustIdent {
    RustIdent::create(&format!("{obj_name}_id"))
}
