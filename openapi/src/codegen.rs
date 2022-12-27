use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::iter::FromIterator;

use heck::{CamelCase, SnakeCase};
use openapiv3::{
    AdditionalProperties, Parameter, ParameterSchemaOrContent, PathStyle, ReferenceOr, Schema,
    SchemaKind, Type,
};

use crate::spec::{
    as_any_of_first_item_title, as_data_array_item, as_enum_strings, as_first_enum_value,
    as_object_enum_name, as_object_properties, as_object_type, err_schema_expected,
    find_param_by_name, get_id_param, get_ok_response, get_ok_response_schema,
    get_request_form_parameters, non_path_ref_params, ExpansionResources,
};
use crate::templates::derives::Derives;
use crate::templates::enums::{EnumTemplateData, EnumVariant};
use crate::templates::enums_with_fields::{EnumWithFieldData, FieldedEnumVariant};
use crate::templates::parameters::ParamData;
use crate::templates::prelude::{write_prelude, CrateImports};
use crate::templates::requests::{gen_impl_object, RequestBase, RequestData, RequestsTemplateData};
use crate::templates::structs::{StructField, StructTemplateData};
use crate::{
    file_generator::FileGenerator,
    metadata::Metadata,
    types::{
        InferredEnum, InferredObject, InferredParams, InferredStruct, InferredUnion, MethodTypes,
        TypeError,
    },
    url_finder::UrlFinder,
    util::infer_integer_type,
};

#[tracing::instrument(skip_all, fields(name = %state.name))]
pub fn gen_struct(
    state: &mut FileGenerator,
    meta: &Metadata,

    shared_objects: &mut BTreeSet<FileGenerator>,
    url_finder: &UrlFinder,
) -> String {
    let name = state.name.clone();
    let object = &name;
    let struct_derives = [
        Derives::Clone,
        Derives::Debug,
        Derives::Default,
        Derives::Deserialize,
        Derives::Serialize,
    ];

    let id_type = meta.schema_to_id_type(object);
    let struct_name = meta.schema_to_rust_type(object);
    let schema = meta.spec.get_schema_unwrapped(object).as_item().expect("Expected item");
    let obj = as_object_type(schema).expect("Expected object type");
    let schema_title = schema.schema_data.title.as_ref().expect("No title found");
    let deleted_schema = meta.spec.component_schemas().get(&format!("deleted_{}", object));

    log::trace!("struct {} {{...}}", struct_name);

    // Generate the struct type
    let mut doc_comment = format!(r#"The resource representing a Stripe "{schema_title}"."#);
    if let Some(doc_url) = url_finder.url_for_object(object) {
        let _ = writeln!(doc_comment, "\n\nFor more details see <{doc_url}>");
    }
    let mut struct_template_data =
        StructTemplateData::new(&struct_name).doc(&doc_comment).derives(&struct_derives);
    if let Some((id_type, _)) = &id_type {
        state.use_ids.insert(id_type.clone());
        let mut struct_field = StructField::new("id", id_type);
        if let Some(doc) = obj
            .properties
            .get("id")
            .and_then(|id| id.as_item().and_then(|i| i.schema_data.description.as_ref()))
        {
            struct_field = struct_field.doc(doc);
        }
        if id_type == "InvoiceId" {
            struct_field = struct_field.apply_specific_serde_default("InvoiceId::none");
        }
        struct_template_data.add_field(struct_field);
    }
    let mut did_emit_deleted = false;
    for (key, field) in &obj.properties {
        if key == "id" || key == "object" {
            continue;
        }
        if !did_emit_deleted
            && deleted_schema.is_some()
            && key.as_str().cmp("deleted") == std::cmp::Ordering::Greater
        {
            let struct_field = StructField::new("deleted", "bool")
                .doc("Always true for a deleted object")
                .serde_default();
            struct_template_data.add_field(struct_field);
            did_emit_deleted = true;
        }
        let required = obj.required.contains(key);
        let force_optional = if let Some(properties) =
            deleted_schema.and_then(|schema| schema.as_item()).and_then(as_object_properties)
        {
            !properties.contains_key(key)
        } else {
            false
        };
        let field = gen_field(
            state,
            meta,
            object,
            key,
            field,
            required && !force_optional,
            false,
            shared_objects,
        );
        struct_template_data.add_field(field);
    }
    struct_template_data.gen_definition()
}

#[tracing::instrument(skip_all)]
pub fn gen_prelude(state: &FileGenerator, meta: &Metadata) -> String {
    let name = state.name.clone();
    let object = &name;

    let prelude_imports = vec![
        CrateImports {
            path: "client",
            imports: Vec::from_iter(state.use_config.iter()).into_iter().copied().collect(),
        },
        CrateImports {
            path: "ids",
            imports: Vec::from_iter(state.use_ids.iter()).into_iter().map(|s| s.as_str()).collect(),
        },
        CrateImports {
            path: "params",
            imports: Vec::from_iter(state.use_params.iter()).into_iter().copied().collect(),
        },
        CrateImports {
            path: "resources",
            imports: state
                .use_resources
                .iter()
                .filter(|&x| {
                    state.generated_schemas.keys().all(|sch| x != &meta.schema_to_rust_type(sch))
                        && !state.inferred_parameters.contains_key(x)
                        && !state.inferred_structs.contains_key(x)
                        && !state.inferred_unions.contains_key(x)
                        && !state.inferred_enums.contains_key(x)
                        && x != &meta.schema_to_rust_type(object)
                })
                .map(|s| s.as_str())
                .collect::<Vec<_>>(),
        },
    ];
    write_prelude(prelude_imports)
}

#[tracing::instrument(skip_all)]
pub fn gen_generated_schemas(
    state: &mut FileGenerator,
    meta: &Metadata,
    shared_objects: &mut BTreeSet<FileGenerator>,
) -> String {
    let mut out = vec![];
    while let Some(schema_name) =
        state.generated_schemas.iter().find_map(|(k, &v)| if !v { Some(k) } else { None }).cloned()
    {
        let struct_name = meta.schema_to_rust_type(&schema_name);
        let derives = [
            Derives::Clone,
            Derives::Debug,
            Derives::Default,
            Derives::Deserialize,
            Derives::Serialize,
        ];
        let mut template_data = StructTemplateData::new(struct_name).derives(&derives);
        let schema = meta.spec.get_schema_unwrapped(&schema_name).as_item().expect("Expected item");
        let obj_type = as_object_type(schema).expect("Expected object type schema");
        for (key, field) in &obj_type.properties {
            let required = obj_type.required.contains(key);
            let struct_field =
                gen_field(state, meta, &schema_name, key, field, required, false, shared_objects);
            template_data.add_field(struct_field);
        }
        out.push(template_data.gen_definition());

        // Set the schema to generated
        *state.generated_schemas.entry(schema_name).or_default() = true;
    }
    out.join("\n")
}

#[tracing::instrument(skip_all)]
pub fn gen_multitype_params(
    parent_struct_rust_type: &str,
    param_name: &str,
    param: &Parameter,
    state: &mut FileGenerator,
    meta: &Metadata,
) -> ParamData {
    let member_schema = match &param.parameter_data_ref().format {
        ParameterSchemaOrContent::Schema(ReferenceOr::Item(s)) => s,
        _ => panic!("Expected schema content"),
    };
    match gen_member_variable_string(member_schema) {
        Ok(type_) => ParamData::new(type_),
        Err(TypeError::NoType) => {
            //Weird case, found with anyOf so only case we are handling
            //at the current moment
            if let SchemaKind::AnyOf { .. } = &member_schema.schema_kind {
                let union_addition = format!("{param_name}_union");
                let mut new_type_name = parent_struct_rust_type.to_owned();
                new_type_name.push_str(&meta.schema_to_rust_type(&union_addition));

                let inferred_object = InferredObject {
                    rust_type: new_type_name.clone(),
                    schema: member_schema.clone(),
                };
                state.generated_objects.insert(new_type_name.clone(), inferred_object);
                ParamData::new(new_type_name)
            } else {
                panic!("Strange case, haven't handled this yet: {:#?}", member_schema);
            }
        }
        Err(TypeError::IsObject) => {
            let new_type_name =
                member_schema.schema_data.title.as_ref().map(|s| s.to_camel_case()).unwrap_or_else(
                    || format!("{}{}Info", parent_struct_rust_type, param_name.to_camel_case()),
                );

            let inferred_object =
                InferredObject { rust_type: new_type_name.clone(), schema: member_schema.clone() };
            state.generated_objects.insert(new_type_name.clone(), inferred_object);
            ParamData::new(new_type_name)
        }
        _ => {
            panic!("Don't recognize this: {:#?}", member_schema);
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_inferred_params(
    state: &mut FileGenerator,
    meta: &Metadata,
    shared_objects: &mut BTreeSet<FileGenerator>,
) -> String {
    let object = state.name.clone();
    let id_type = meta.schema_to_id_type(&object);
    let struct_name = meta.schema_to_rust_type(&object);
    let mut params_out = vec![];

    for (_, params) in state.inferred_parameters.clone() {
        let params_schema = params.rust_type.to_snake_case();

        // Derive Default when no param is required
        let can_derive_default =
            params.parameters.iter().all(|param| !param.parameter_data_ref().required);
        let mut derives = vec![Derives::Clone, Derives::Debug, Derives::Serialize];
        if can_derive_default {
            derives.push(Derives::Default);
        }
        let mut struct_data = StructTemplateData::new(&params.rust_type)
            .doc(format!("The parameters for `{struct_name}::{}`.\n", params.method))
            .derives(&derives)
            .lifetime("'a");
        for param in params.parameters.iter().filter(|p| matches!(p, Parameter::Query { .. })) {
            let param_name = param.parameter_data_ref().name.as_str();
            let param_rename = match param_name {
                "type" => "type_",
                other => other,
            };
            let required = param.parameter_data_ref().required;
            let initializer = match param_name {
                // TODO: Handle these unusual params
                "bank_account" | "usage" => continue,
                "destination" | "card" => {
                    gen_multitype_params(&params.rust_type, param_name, param, state, meta)
                }
                "product" => {
                    state.use_params.insert("IdOrCreate");
                    state.use_resources.insert("CreateProduct".to_owned());
                    ParamData::new("IdOrCreate<'a, CreateProduct<'a>>")
                }
                "metadata" => {
                    state.use_params.insert("Metadata");
                    ParamData::new("Metadata")
                }
                "expand" => {
                    state.use_params.insert("Expand");
                    ParamData::new("&'a [&'a str]").skip_serializing("Expand::is_empty")
                }
                "limit" => ParamData::new("u64"),
                param @ "starting_after" | param @ "ending_before" => {
                    let cursor_type =
                        id_type.as_ref().map(|(x, _)| x.as_str()).unwrap_or("&'a str");
                    if required {
                        panic!("unexpected \"required\" `{}` parameter", param);
                    }
                    ParamData::new(cursor_type)
                }
                _ => {
                    let schema = match &param.parameter_data_ref().format {
                        ParameterSchemaOrContent::Schema(s) => s,
                        ParameterSchemaOrContent::Content(_) => panic!("Expected schema"),
                    };
                    let kind = schema.as_item().map(|s| &s.schema_kind);
                    let is_string_schema = matches!(kind, Some(SchemaKind::Type(Type::String(_))));
                    if let Some((use_path, rust_type)) =
                        meta.field_to_rust_type(params_schema.as_str(), param_name)
                    {
                        state.add_use(use_path);
                        ParamData::new(rust_type)
                    } else if meta.schema_to_id_type(param_name).is_some()
                        && is_string_schema
                        && param_name != "tax_id"
                    {
                        let (id_type, _) = meta.schema_to_id_type(param_name).unwrap();
                        state.use_ids.insert(id_type.clone());
                        ParamData::new(id_type)
                    } else if matches!(kind, Some(SchemaKind::Type(Type::Boolean { .. }))) {
                        ParamData::new("bool")
                    } else if let Some(SchemaKind::Type(Type::Integer(int_type))) = kind {
                        ParamData::new(infer_integer_type(state, param_name, &int_type.format))
                    } else if matches!(kind, Some(SchemaKind::Type(Type::Number(_)))) {
                        ParamData::new("f64")
                    } else if schema.as_item().and_then(as_any_of_first_item_title)
                        == Some("range_query_specs")
                    {
                        state.use_params.insert("RangeQuery");
                        state.use_params.insert("Timestamp");
                        ParamData::new("RangeQuery<Timestamp>")
                    } else if let Some(enum_strings) = schema.as_item().and_then(as_enum_strings) {
                        let enum_schema = meta.schema_field(&object, param_rename);
                        let enum_name = meta.schema_to_rust_type(&enum_schema);
                        let enum_ = InferredEnum {
                            parent: params.rust_type.clone(),
                            field: param_rename.into(),
                            options: enum_strings,
                        };
                        let inserted = state.try_insert_enum(enum_name.clone(), enum_.clone());
                        let enum_name = if inserted.is_err() {
                            let enum_schema = format!("{}_filter", enum_schema);
                            let enum_name = meta.schema_to_rust_type(&enum_schema);
                            state.insert_enum(enum_name.clone(), enum_);
                            enum_name
                        } else {
                            enum_name
                        };

                        ParamData::new(enum_name)
                    } else if (param_name == "currency" || param_name.ends_with("_currency"))
                        && is_string_schema
                    {
                        state.use_resources.insert("Currency".into());
                        ParamData::new("Currency")
                    } else if is_string_schema {
                        ParamData::new("&'a str")
                    } else if schema.as_item().is_none()
                        || matches!(
                            kind,
                            Some(
                                SchemaKind::AnyOf { .. }
                                    | SchemaKind::Type(Type::Object(_))
                                    | SchemaKind::Type(Type::Array(_))
                            )
                        )
                    {
                        let rust_type = gen_field_rust_type(
                            state,
                            meta,
                            &params.rust_type.to_snake_case(),
                            param_rename,
                            schema,
                            required,
                            false,
                            shared_objects,
                        );
                        ParamData::new(rust_type)
                    } else if required {
                        panic!("error: skipped required parameter: {} => {:?}", param_name, schema);
                    } else {
                        log::warn!("skipping optional parameter: {}", param_name);
                        continue;
                    }
                }
            };
            let rust_type = if !required
                && !initializer.rust_type.starts_with("Option<")
                // Special-case expand (for now) since its optional handling is unique
                && param_name != "expand"
            {
                format!("Option<{}>", initializer.rust_type)
            } else {
                initializer.rust_type
            };
            let mut struct_field = StructField::new(param_rename, &rust_type).required(required);
            if let Some(skip_ser) = initializer.skip_serializing {
                struct_field = struct_field.skip_serializing(skip_ser);
            } else if rust_type.starts_with("Option<") {
                struct_field = struct_field.skip_serializing("Option::is_none");
            }
            if let Some(doc) = &param.parameter_data_ref().description {
                struct_field = struct_field.doc(doc);
            }
            if param_rename != param_name {
                struct_field = struct_field.rename_as(param_name);
            }
            struct_data.add_field(struct_field);
        }
        params_out.push(format!(
            "{}\n{}",
            struct_data.gen_definition(),
            struct_data.gen_constructor()
        ));
    }
    params_out.join("\n")
}

#[tracing::instrument(skip_all)]
pub fn gen_emitted_structs(
    state: &mut FileGenerator,
    meta: &Metadata,
    shared_objects: &mut BTreeSet<FileGenerator>,
) -> String {
    let mut struct_out = vec![];
    let mut emitted_structs = BTreeSet::new();
    while state
        .inferred_structs
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
        .difference(&emitted_structs)
        .any(|_| true)
    {
        for (struct_name, struct_) in state.inferred_structs.clone() {
            if emitted_structs.contains(&struct_name) {
                continue;
            } else {
                emitted_structs.insert(struct_name.clone());
                log::trace!("struct {} {{ ... }}", struct_name);
            }

            let obj = match struct_.schema.schema_kind {
                SchemaKind::Type(Type::Object(o)) => o,
                _ => {
                    // TODO: Handle these objects
                    log::warn!("{} has no properties ({:#?})", struct_name, struct_.schema);
                    continue;
                }
            };

            let rust_struct_name = struct_name.to_camel_case();
            let mut struct_template_data = StructTemplateData::new(rust_struct_name).derives(&[
                Derives::Clone,
                Derives::Debug,
                Derives::Default,
                Derives::Deserialize,
                Derives::Serialize,
            ]);
            obj.properties.iter().for_each(|(key, field)| {
                struct_template_data.add_field(gen_field(
                    state,
                    meta,
                    &struct_name.to_snake_case(),
                    key,
                    field,
                    obj.required.contains(key),
                    false,
                    shared_objects,
                ))
            });
            struct_out.push(struct_template_data.gen_definition());
        }
    }
    struct_out.join("\n")
}

#[tracing::instrument(skip_all)]
pub fn gen_unions(unions: &BTreeMap<String, InferredUnion>, meta: &Metadata) -> String {
    let mut unions_out = vec![];
    for (union_name, union_) in unions {
        log::trace!("union {} {{ ... }}", union_name);

        let enum_name = union_name.to_camel_case();
        let mut enum_data = EnumWithFieldData::new(enum_name);
        for variant_schema in &union_.schema_variants {
            let schema =
                meta.spec.get_schema_unwrapped(variant_schema).as_item().expect("Expected an item");
            let object_name = as_object_enum_name(schema)
                .unwrap_or_else(|| schema.schema_data.title.clone().unwrap());
            let variant_name = meta.schema_to_rust_type(&object_name);
            let mut variant =
                FieldedEnumVariant::new(&variant_name, meta.schema_to_rust_type(variant_schema));
            if variant_name.to_snake_case() != object_name {
                variant = variant.rename_as(object_name);
            }
            enum_data.add_variant(variant);
        }
        if let Some(first_variant) = enum_data.variants.first().cloned() {
            enum_data = enum_data.with_default_variant(first_variant.variant_name);
        }
        unions_out.push(enum_data.gen_definition_and_methods());
    }
    unions_out.join("\n")
}

#[tracing::instrument(skip_all)]
pub fn gen_variant_name(wire_name: &str, meta: &Metadata) -> String {
    match wire_name {
        "*" => "All".to_string(),
        n => {
            if n.chars().next().unwrap().is_digit(10) {
                format!("V{}", n.to_string().replace('-', "_").replace('.', "_"))
            } else {
                meta.schema_to_rust_type(wire_name)
            }
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_enums(enums: &BTreeMap<String, InferredEnum>, meta: &Metadata) -> String {
    let mut enums_out = vec![];
    for (enum_name, enum_) in enums {
        log::trace!("enum {} {{ ... }}", enum_name);

        let doc_comment = format!(
            "An enum representing the possible values of an `{}`'s `{}` field",
            enum_.parent, enum_.field
        );
        let mut enum_data = EnumTemplateData::new(enum_name, doc_comment);
        for wire_name in &enum_.options {
            if wire_name.trim().is_empty() {
                continue;
            }
            let variant_name = gen_variant_name(wire_name.as_str(), meta);
            if variant_name.trim().is_empty() {
                panic!("unhandled enum variant: {:?}", wire_name)
            }
            enum_data.add_variant(EnumVariant { wire_name, variant_name });
        }

        if let Some(first_variant) = enum_data.variants.first().cloned() {
            enum_data = enum_data.with_default_variant(first_variant.variant_name)
        }
        enums_out.push(enum_data.gen_definition_and_methods());
    }
    enums_out.join("\n")
}

#[tracing::instrument(skip_all)]
pub fn gen_member_variable_string(schema: &Schema) -> Result<String, TypeError> {
    if let SchemaKind::Type(typ) = &schema.schema_kind {
        match typ {
            Type::String(_) => Ok("String".into()),
            Type::Integer(_) => Ok("i32".into()),
            Type::Boolean { .. } => Ok("bool".into()),
            Type::Array(arr) => Ok(format!(
                "Vec<{}>",
                gen_member_variable_string(arr.items.as_ref().unwrap().as_item().unwrap()).unwrap()
            )),
            Type::Object(_) => Err(TypeError::IsObject),
            _ => Err(TypeError::Unhandled),
        }
    } else {
        Err(TypeError::NoType)
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_objects(objects: &BTreeMap<String, InferredObject>) -> String {
    let mut generated_objects = objects.clone();
    let mut objects_out = vec![];
    while !generated_objects.is_empty() {
        let key_str: String;
        let value_obj: InferredObject;
        {
            let (key, value) = generated_objects.iter().next().unwrap();
            log::trace!("object: {} -- {:#?}", key, value);
            key_str = key.clone();
            value_obj = value.clone();
        }

        //Okay, we need something more general for these common
        //cases.  Hopefully, they aren't too common.  Right now, we
        //don't have a clear way to handle 2nd level nested new types
        if key_str == "Metadata" {
            generated_objects.remove(&key_str);
            continue;
        }

        let schema = value_obj.schema.clone();
        let doc_comment = schema.schema_data.description.clone().unwrap_or_default();

        match schema.schema_kind {
            SchemaKind::Type(Type::Object(obj)) => {
                let mut struct_data = StructTemplateData::new(&key_str)
                    .derives(&[
                        Derives::Clone,
                        Derives::Debug,
                        Derives::Default,
                        Derives::Deserialize,
                        Derives::Serialize,
                    ])
                    .doc(&doc_comment);
                let props = obj.properties.iter().flat_map(|(name, schema_or)| match schema_or {
                    ReferenceOr::Reference { .. } => None,
                    ReferenceOr::Item(schema) => Some((name, schema)),
                });
                for (member_name, member_schema) in props {
                    let is_required = obj.required.contains(member_name);
                    let field_doc_comment =
                        member_schema.schema_data.description.clone().unwrap_or_default();
                    let base_rust_type = match gen_member_variable_string(member_schema) {
                        Ok(normal_var) => normal_var,
                        Err(TypeError::IsObject) => {
                            let rust_type = member_name.to_camel_case();
                            let new_params = InferredObject {
                                rust_type: rust_type.clone(),
                                schema: *member_schema.clone(),
                            };
                            generated_objects.insert(rust_type.clone(), new_params);
                            rust_type
                        }
                        _ => {
                            panic!("Unhandled case, inspect: {:#?}", member_schema);
                        }
                    };
                    let (rust_type, skip_ser) = if is_required {
                        (base_rust_type, None)
                    } else {
                        (format!("Option<{base_rust_type}>"), Some("Option::is_none"))
                    };
                    struct_data.add_field(
                        StructField::new(member_name, rust_type)
                            .doc(field_doc_comment)
                            .set_skip_serializing(skip_ser),
                    );
                }
                objects_out.push(struct_data.gen_definition());
            }
            SchemaKind::AnyOf { any_of } => {
                let mut enum_data = EnumWithFieldData::new(&key_str).doc(doc_comment);
                let mut variants_count = HashMap::new();

                for value in any_of.iter().flat_map(|a| match a {
                    ReferenceOr::Reference { .. } => None,
                    ReferenceOr::Item(s) => Some(s),
                }) {
                    let title = &value.schema_data.title;
                    let type_name = gen_member_variable_string(value).unwrap_or_else(|_| {
                        let type_name = title.as_ref().unwrap().to_camel_case();

                        generated_objects.insert(
                            type_name.clone(),
                            InferredObject { rust_type: type_name.clone(), schema: value.clone() },
                        );

                        type_name
                    });

                    // if the title is not provided, the variant
                    // should have the same name as the type it contains
                    // with an optional suffix if there are clashes
                    let variant_name =
                        title.as_ref().map(|s| s.to_camel_case()).unwrap_or_else(|| {
                            let count = variants_count.entry(type_name.clone()).or_insert(0);
                            let suffix =
                                if *count == 0 { "".to_string() } else { count.to_string() };
                            *count += 1;

                            format!("{type_name}{suffix}")
                        });

                    enum_data.add_variant(FieldedEnumVariant::new(variant_name, type_name));
                }
                objects_out.push(enum_data.gen_definition_and_methods());
            }
            other => panic!("Expected an object here got: {:?}", other),
        }
        generated_objects.remove(&key_str);
    }
    objects_out.join("\n")
}

#[tracing::instrument(skip_all)]
pub fn gen_field<'a, T: Borrow<Schema>>(
    state: &mut FileGenerator,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &ReferenceOr<T>,
    required: bool,
    default: bool,
    shared_objects: &mut BTreeSet<FileGenerator>,
) -> StructField {
    let rust_type = gen_field_rust_type(
        state,
        meta,
        object,
        field_name,
        field,
        required,
        default,
        shared_objects,
    );
    let mut field_rename = field_name.to_snake_case();
    if field_rename == "type" {
        field_rename = "type_".into();
    }
    let mut struct_field = StructField::new(&field_rename, &rust_type);
    if let Some(doc) = field.as_item().and_then(|s| s.borrow().schema_data.description.as_deref()) {
        struct_field = struct_field.doc(doc);
    }
    if field_rename != field_name {
        struct_field = struct_field.rename_as(field_name);
    }
    if !required {
        if rust_type == "bool" || rust_type == "Metadata" || rust_type.starts_with("List<") {
            struct_field = struct_field.serde_default();
        } else if rust_type.starts_with("Option<") {
            struct_field = struct_field.skip_serializing("Option::is_none");
        }
    }
    struct_field
}

fn gen_schema_ref(
    state: &mut FileGenerator,
    meta: &Metadata,
    object: &str,
    path: &str,
    shared_objects: &mut BTreeSet<FileGenerator>,
) -> String {
    let schema_name = path.trim_start_matches("#/components/schemas/");
    let type_name = meta.schema_to_rust_type(schema_name);
    if schema_name != object {
        if meta.objects.contains(schema_name) {
            state.use_resources.insert(type_name.clone());
        } else if meta.dependents.get(schema_name).map(|x| x.len()).unwrap_or(0) > 1 {
            state.use_resources.insert(type_name.clone());
            shared_objects.insert(FileGenerator::new(schema_name.to_string()));
        } else if !state.generated_schemas.contains_key(schema_name) {
            state.generated_schemas.insert(schema_name.into(), false);
        }
    }
    type_name
}

fn gen_schema_or_ref_type<T: Borrow<Schema>>(
    state: &mut FileGenerator,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &ReferenceOr<T>,
    required: bool,
    default: bool,
    shared_objects: &mut BTreeSet<FileGenerator>,
) -> String {
    match field {
        ReferenceOr::Reference { reference } => {
            gen_schema_ref(state, meta, object, reference, shared_objects)
        }
        ReferenceOr::Item(schema) => gen_field_type(
            state,
            meta,
            object,
            field_name,
            schema.borrow(),
            required,
            default,
            shared_objects,
        ),
    }
}

#[tracing::instrument(skip_all)]
fn gen_field_type(
    state: &mut FileGenerator,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &Schema,
    required: bool,
    default: bool,
    shared_objects: &mut BTreeSet<FileGenerator>,
) -> String {
    match &field.schema_kind {
        // N.B. return immediately; if we want to use `Default` for bool rather than `Option`
        SchemaKind::Type(Type::Boolean {}) => "bool".into(),
        SchemaKind::Type(Type::Number(_)) => "f64".into(),
        SchemaKind::Type(Type::Integer(format)) => {
            infer_integer_type(state, field_name, &format.format)
        }
        SchemaKind::Type(Type::String(typ)) => {
            let variants = typ.enumeration.iter().flatten().cloned().collect::<Vec<_>>();
            if !variants.is_empty() {
                let enum_schema = meta.schema_field(object, field_name);
                let enum_name = meta.schema_to_rust_type(&enum_schema);
                let parent_type = meta.schema_to_rust_type(object);
                let enum_ = InferredEnum {
                    parent: parent_type,
                    field: field_name.into(),
                    options: variants,
                };
                state.insert_enum(enum_name.clone(), enum_);
                enum_name
            } else {
                "String".into()
            }
        }
        SchemaKind::Type(Type::Array(typ)) => {
            let element = typ.items.clone().unwrap().unbox();
            let element_type = gen_field_rust_type(
                state,
                meta,
                object,
                field_name,
                &element,
                true,
                false,
                shared_objects,
            );
            format!("Vec<{}>", element_type)
        }
        SchemaKind::Type(Type::Object(typ)) => {
            if as_object_enum_name(field).as_deref() == Some("list") {
                state.use_params.insert("List");
                let element = as_data_array_item(typ).unwrap_or_else(|| {
                    panic!("Expected to find array item but found {:?}", field.schema_kind)
                });
                let element_field_name = if field_name.ends_with('s') {
                    field_name[0..field_name.len() - 1].into()
                } else if field_name.ends_with("ies") {
                    format!("{}y", &field_name[0..field_name.len() - 3])
                } else {
                    field_name.into()
                };
                let element_type = gen_field_rust_type(
                    state,
                    meta,
                    object,
                    &element_field_name,
                    element,
                    true,
                    false,
                    shared_objects,
                );

                // N.B. return immediately; we use `Default` for list rather than `Option`
                return format!("List<{}>", element_type);
            }

            if let Some(AdditionalProperties::Schema(additional)) = &typ.additional_properties {
                return gen_schema_or_ref_type(
                    state,
                    meta,
                    object,
                    field_name,
                    additional,
                    required,
                    default,
                    shared_objects,
                );
            }

            let struct_schema = meta.schema_field(object, field_name);
            let struct_name = meta.schema_to_rust_type(&struct_schema);
            let struct_ = InferredStruct { field: field_name.into(), schema: field.clone() };
            state.insert_struct(struct_name.clone(), struct_);
            struct_name
        }
        SchemaKind::AnyOf { .. } | SchemaKind::OneOf { .. } => {
            let any_of = match &field.schema_kind {
                SchemaKind::AnyOf { any_of } => any_of,
                SchemaKind::OneOf { one_of } => one_of,
                _ => unreachable!(),
            };
            if any_of.len() == 1
                || (any_of.len() == 2
                    && any_of[1]
                        .as_item()
                        .and_then(as_first_enum_value)
                        .map(|v| v.is_empty())
                        .unwrap_or_default())
            {
                gen_field_rust_type(
                    state,
                    meta,
                    object,
                    field_name,
                    &any_of[0],
                    true,
                    false,
                    shared_objects,
                )
            } else if let Some(resources) = field.schema_data.extensions.get("x-expansionResources")
            {
                let expansion_resources =
                    serde_json::from_value::<ExpansionResources>(resources.clone())
                        .expect("Could not deserialize expansion resources");

                let ty_ = gen_field_rust_type(
                    state,
                    meta,
                    object,
                    field_name,
                    &ReferenceOr::Item(Schema {
                        schema_data: Default::default(),
                        schema_kind: expansion_resources.into_schema_kind(),
                    }),
                    true,
                    false,
                    shared_objects,
                );
                state.use_params.insert("Expandable");
                format!("Expandable<{}>", ty_)
            } else if any_of[0].as_item().and_then(|s| s.schema_data.title.as_deref())
                == Some("range_query_specs")
            {
                state.use_params.insert("RangeQuery");
                state.use_params.insert("Timestamp");
                "RangeQuery<Timestamp>".into()
            } else {
                log::trace!("object: {}, field_name: {}", object, field_name);
                let union_addition = format!("{field_name}_union");
                let union_schema = meta.schema_field(object, &union_addition);
                let union_name = meta.schema_to_rust_type(&union_schema);
                log::trace!("union_schema: {}, union_name: {}", union_schema, union_name);
                let union_ = InferredUnion {
                    field: field_name.into(),
                    schema_variants: any_of
                        .iter()
                        .map(|x| {
                            let schema_name = match &x {
                                ReferenceOr::Reference { reference } => {
                                    reference.trim_start_matches("#/components/schemas/")
                                }
                                ReferenceOr::Item(_) => {
                                    panic!(
                                        "invalid union for `{}.{}`:  {:#?}",
                                        object, field_name, field
                                    )
                                }
                            };
                            let type_name = meta.schema_to_rust_type(schema_name);
                            if meta.objects.contains(schema_name)
                                || meta.dependents.get(schema_name).map(|x| x.len()).unwrap_or(0)
                                    > 1
                            {
                                state.use_resources.insert(type_name);
                            } else if !state.generated_schemas.contains_key(schema_name) {
                                state.generated_schemas.insert(schema_name.into(), false);
                            }
                            schema_name.into()
                        })
                        .collect(),
                };
                state.inferred_unions.insert(union_name.clone(), union_);
                union_name
            }
        }
        _ => {
            panic!("unhandled field type for `{}.{}`: {:#?}\n", object, field_name, field)
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_field_rust_type<T: Borrow<Schema>>(
    state: &mut FileGenerator,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &ReferenceOr<T>,
    required: bool,
    default: bool,
    shared_objects: &mut BTreeSet<FileGenerator>,
) -> String {
    let maybe_schema = field.as_item().map(|s| s.borrow());
    let is_nullable = maybe_schema.map(|s| s.schema_data.nullable).unwrap_or_default();
    if let Some((use_path, rust_type)) = meta.field_to_rust_type(object, field_name) {
        state.add_use(use_path);
        return rust_type.into();
    }
    if field_name == "metadata" {
        state.use_params.insert("Metadata");
        return "Metadata".into();
    } else if (field_name == "currency" || field_name.ends_with("_currency"))
        && matches!(maybe_schema.map(|s| &s.schema_kind), Some(SchemaKind::Type(Type::String(_))))
    {
        state.use_resources.insert("Currency".into());
        return if !required || is_nullable {
            "Option<Currency>".into()
        } else {
            "Currency".into()
        };
    } else if field_name == "created" {
        state.use_params.insert("Timestamp");
        return if !required || is_nullable {
            "Option<Timestamp>".into()
        } else {
            "Timestamp".into()
        };
    }

    let ty = gen_schema_or_ref_type(
        state,
        meta,
        object,
        field_name,
        field,
        required,
        default,
        shared_objects,
    );
    if ty == "bool" && default {
        // N.B. return immediately; if we want to use `Default` for bool rather than `Option`
        // Not sure why this is here, but we want to preserve it for now
        return "bool".into();
    }
    if ty.contains("List<") {
        // N.B. return immediately; we use `Default` for list rather than `Option`
        return ty;
    }

    let optional = !required || is_nullable;
    let should_box = ty.as_str() == "ApiErrors";

    match (optional, should_box) {
        (true, true) => format!("Option<Box<{}>>", ty),
        (true, false) => format!("Option<{}>", ty),
        (false, true) => format!("Box<{}>", ty),
        (false, false) => ty,
    }
}

pub fn gen_object_trait(
    state: &mut FileGenerator,
    meta: &Metadata,
    object_literal: &str,
) -> String {
    let object = &state.name;
    let rust_struct = meta.schema_to_rust_type(object);
    let id_type = meta.schema_to_id_type(object);
    let object_id = id_type.as_ref().map(|id| id.0.as_str());
    state.use_params.insert("Object");
    gen_impl_object(&rust_struct, object_literal, object_id)
}

#[tracing::instrument(skip_all, fields(name = %state.name))]
pub fn gen_impl_requests(state: &mut FileGenerator, meta: &Metadata) -> Option<(String, String)> {
    let object = &state.name;
    let requests = meta.requests.get(object)?;
    let rust_struct = meta.schema_to_rust_type(object);
    let id_type = meta.schema_to_id_type(object);
    let object_id = id_type.as_ref().map(|id| id.0.as_str());
    log::trace!("impl {} {{ ... }}", rust_struct);

    let mut request_templates =
        RequestsTemplateData { methods: HashMap::new(), parent_struct: rust_struct.clone() };

    for path in requests {
        // Unwrapped is safe here to avoid dealing with an `Option` since these paths come
        // from the spec already
        let request = meta
            .spec
            .get_request_unwrapped(path)
            .as_item()
            .expect("Expected item, not path reference");
        let segments =
            path.trim_start_matches("/v1/").split('/').map(String::from).collect::<Vec<_>>();

        if let Some(get_request) = &request.get {
            let ok_resp = match get_ok_response(get_request) {
                None => continue,
                Some(s) => s,
            };
            if !err_schema_expected(get_request) {
                continue; // skip generating this unusual request (for now...)
            }
            let doc_comment =
                get_request.description.as_ref().expect("No description in GET request");
            let body_schema = ok_resp
                .as_item()
                .and_then(|s| s.content.get("application/json"))
                .and_then(|c| c.schema.as_ref());
            if body_schema
                .as_ref()
                .and_then(|s| s.as_item())
                .map(|s| as_object_enum_name(s).as_deref() == Some("list"))
                .unwrap_or_default()
                && !request_templates.methods.contains_key(&MethodTypes::List)
            {
                let params_name = if rust_struct.ends_with('y') {
                    format!("List{}ies", &rust_struct[0..rust_struct.len() - 1])
                } else {
                    format!("List{rust_struct}s")
                };
                let params = InferredParams {
                    method: "list".into(),
                    rust_type: params_name.clone(),
                    parameters: non_path_ref_params(get_request),
                };
                state.inferred_parameters.insert(params_name.to_snake_case(), params);
                state.use_params.insert("List");
                state.use_params.insert("Paginable");
                request_templates.add(RequestData::new(
                    RequestBase::List { params_name },
                    doc_comment,
                    segments.clone(),
                ));
            } else if segments.len() == 2
                && !request_templates.methods.contains_key(&MethodTypes::Retrieve)
            {
                let id_param = match get_id_param(&get_request.parameters) {
                    Some(p) => p,
                    None => continue,
                };
                let expand_param = find_param_by_name(get_request, "expand");
                if let Some(id_type) = &object_id {
                    assert!(id_param.parameter_data_ref().required);
                    assert!(matches!(id_param, Parameter::Path { style: PathStyle::Simple, .. }));

                    let has_expand_param = if let Some(param) = expand_param {
                        state.use_params.insert("Expand");
                        assert!(matches!(param, Parameter::Query { .. }));
                        true
                    } else {
                        false
                    };
                    request_templates.add(RequestData::new(
                        RequestBase::Retrieve { id_type: id_type.to_string(), has_expand_param },
                        doc_comment,
                        segments.clone(),
                    ));
                }
            }
        }

        if let Some(post_request) = &request.post {
            let return_type = match get_ok_response_schema(post_request) {
                Some(ReferenceOr::Reference { reference }) => {
                    let schema = reference.trim_start_matches("#/components/schemas/");
                    meta.schema_to_rust_type(schema)
                }
                _ => continue,
            };
            if !err_schema_expected(post_request) {
                continue; // skip generating this unusual request (for now...)
            }
            let doc_comment =
                post_request.description.as_ref().expect("No description for POST request");
            let parameter_count = post_request.parameters.len();

            let create = doc_comment.contains("Create")
                || doc_comment.contains("create")
                || doc_comment.contains("Adds")
                || doc_comment.contains("adds");

            let update = doc_comment.contains("Update") || doc_comment.contains("update");

            if !request_templates.methods.contains_key(&MethodTypes::Create)
                && parameter_count == 0
                && create
            {
                // Construct `parameters` from the request body schema
                let create_parameters =
                    get_request_form_parameters(post_request).unwrap_or_else(|err| {
                        panic!("Could not extract create parameters due to error {}", err)
                    });
                let params_name = format!("Create{}", rust_struct);
                let params = InferredParams {
                    method: "create".into(),
                    rust_type: params_name.clone(),
                    parameters: create_parameters,
                };
                state.inferred_parameters.insert(params_name.to_snake_case(), params);
                request_templates.add(RequestData::new(
                    RequestBase::Create { params_name, return_type },
                    doc_comment,
                    segments.clone(),
                ));
            } else if !request_templates.methods.contains_key(&MethodTypes::Update)
                && parameter_count == 1
                && update
            {
                // Get the id parameter
                let id_param = match get_id_param(&post_request.parameters) {
                    Some(p) => p,
                    None => continue,
                };

                // Construct `parameters` from the request body schema
                let update_parameters =
                    get_request_form_parameters(post_request).unwrap_or_else(|err| {
                        panic!("Could not extract update parameters due to error {}", err)
                    });
                let params_name = format!("Update{}", rust_struct);
                let params = InferredParams {
                    method: "update".into(),
                    rust_type: params_name.clone(),
                    parameters: update_parameters,
                };
                state.inferred_parameters.insert(params_name.to_snake_case(), params);

                if let Some(id_type) = &object_id {
                    assert!(id_param.parameter_data_ref().required);
                    assert!(matches!(id_param, Parameter::Path { style: PathStyle::Simple, .. }));
                    request_templates.add(RequestData::new(
                        RequestBase::Update {
                            id_type: id_type.to_string(),
                            params_name,
                            return_type,
                        },
                        doc_comment,
                        segments.clone(),
                    ));
                }
            } else {
                log::warn!(
                    "unhandled {} for {rust_struct}: POST {path} (already have {:?})",
                    match (create, update) {
                        (true, _) => "CREATE",
                        (_, true) => "UPDATE",
                        _ => "UNKNOWN",
                    },
                    request_templates.methods.keys()
                );
            }
        }

        if let Some(delete_request) = &request.delete {
            match get_ok_response_schema(delete_request) {
                None => continue,
                Some(s) => s,
            };
            if !err_schema_expected(delete_request) {
                continue; // skip generating this unusual request (for now...)
            }

            let doc_comment =
                delete_request.description.as_ref().expect("Missing DELETE description");
            if segments.len() == 2 && !request_templates.methods.contains_key(&MethodTypes::Delete)
            {
                let id_param = match get_id_param(&delete_request.parameters) {
                    Some(p) => p,
                    None => continue,
                };
                if let Some(id_type) = &object_id {
                    state.use_params.insert("Deleted");
                    assert!(id_param.parameter_data_ref().required);
                    assert!(matches!(id_param, Parameter::Path { style: PathStyle::Simple, .. }));
                    request_templates.add(RequestData::new(
                        RequestBase::Delete { id_type: id_type.to_string() },
                        doc_comment,
                        segments.clone(),
                    ));
                }
            } else {
                log::warn!("unhandled DELETE for {rust_struct}: {path}");
            }
        }
    }
    if request_templates.methods.is_empty() {
        None
    } else {
        state.use_config.insert("Client");
        state.use_config.insert("Response");

        let request_impls = request_templates.gen_requests();
        let impl_paginable = request_templates.maybe_gen_impl_paginable().unwrap_or_default();
        Some((request_impls, impl_paginable))
    }
}
