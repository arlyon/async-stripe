use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;

use heck::{CamelCase, SnakeCase};
use indoc::writedoc;
use openapiv3::{
    AdditionalProperties, Parameter, ParameterSchemaOrContent, PathStyle, ReferenceOr, Schema,
    SchemaKind, Type,
};
use tracing::trace;

use crate::spec::{
    as_any_of_first_item_title, as_data_array_item, as_enum_strings, as_first_enum_value,
    as_object_enum_name, as_object_properties, as_object_type, err_schema_expected,
    find_param_by_name, get_id_param, get_ok_response, get_ok_response_schema,
    get_request_form_parameters, non_path_ref_params, ExpansionResources,
};
use crate::util::{print_doc_from_schema, write_serde_rename};
use crate::{
    file_generator::FileGenerator,
    metadata::Metadata,
    types::{
        InferredEnum, InferredObject, InferredParams, InferredStruct, InferredUnion, MethodTypes,
        TypeError,
    },
    url_finder::UrlFinder,
    util::{infer_integer_type, print_doc_comment, write_out_field},
};

#[tracing::instrument(skip_all, fields(name = %state.name))]
pub fn gen_struct(
    out: &mut String,
    state: &mut FileGenerator,
    meta: &Metadata,

    shared_objects: &mut BTreeSet<FileGenerator>,
    url_finder: &UrlFinder,
) {
    let name = state.name.clone();
    let object = &name;

    let id_type = meta.schema_to_id_type(object);
    let struct_name = meta.schema_to_rust_type(object);
    let schema = meta.spec.get_schema_unwrapped(object).as_item().expect("Expected item");
    let obj = as_object_type(schema).expect("Expected object type");
    let schema_title = schema.schema_data.title.as_ref().unwrap_or_else(|| {
        tracing::warn!("{} has no title", object);
        &object
    });

    let deleted_schema = meta.spec.component_schemas().get(&format!("deleted_{}", object));
    let deleted_properties =
        deleted_schema.and_then(|schema| schema.as_item()).and_then(as_object_properties);

    trace!("struct {} {{...}} ({})", struct_name, name);

    // Generate the struct type
    out.push_str("/// The resource representing a Stripe \"");
    out.push_str(schema_title);
    out.push_str("\".\n");
    if let Some(doc_url) = url_finder.url_for_object(object) {
        out.push_str("///\n");
        out.push_str("/// For more details see <");
        out.push_str(&doc_url);
        out.push_str(">\n");
    }
    out.push_str("#[derive(Clone, Debug, Default, Deserialize, Serialize)]\n");
    out.push_str("pub struct ");
    out.push_str(&struct_name);
    out.push_str(" {\n");
    if let Some((id_type, _)) = &id_type {
        state.use_ids.insert(id_type.clone());
        if let Some(doc) = obj
            .properties
            .get("id")
            .and_then(|id| id.as_item().and_then(|i| i.schema_data.description.as_ref()))
        {
            print_doc_comment(out, doc, 1);
        }
        if id_type == "InvoiceId" {
            out.push_str("    #[serde(default = \"InvoiceId::none\")]");
        }
        out.push_str("    pub id: ");
        out.push_str(id_type);
        out.push_str(",\n");
    }

    let mut did_emit_deleted = false;
    for (key, field) in obj
        .properties
        .iter()
        // we handle id and object separately
        .filter(|(key, _)| !["id", "object"].contains(&key.as_str()))
    {
        // emit deleted in the appropirate place alphabetically
        if !did_emit_deleted
            && deleted_schema.is_some()
            && key.as_str().cmp("deleted") == std::cmp::Ordering::Greater
        {
            out.push('\n');
            out.push_str("    // Always true for a deleted object\n");
            out.push_str("    #[serde(default)]\n");
            out.push_str("    pub deleted: bool,\n");
            did_emit_deleted = true;
        }

        // in the required list and not in the deleted list
        let required = obj.required.contains(key)
            && deleted_properties.map(|map| map.contains_key(key)).unwrap_or(true);

        out.push('\n');
        out.push_str(&gen_field(state, meta, object, key, field, required, false, shared_objects));
    }
    out.push_str("}\n");
}

#[tracing::instrument(skip_all)]
pub fn gen_prelude(state: &FileGenerator, meta: &Metadata) -> String {
    let name = state.name.clone();
    let object = &name;

    let mut prelude = String::new();
    prelude.push_str("// ======================================\n");
    prelude.push_str("// This file was automatically generated.\n");
    prelude.push_str("// ======================================\n\n");
    if !state.use_config.is_empty() {
        prelude.push_str("use crate::client::{");
        for (n, type_) in state.use_config.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(type_);
        }
        prelude.push_str("};\n");
    }
    if !state.use_ids.is_empty() {
        prelude.push_str("use crate::ids::{");
        for (n, type_) in state.use_ids.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(type_);
        }
        prelude.push_str("};\n");
    }
    if !state.use_params.is_empty() {
        prelude.push_str("use crate::params::{");
        for (n, type_) in state.use_params.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(type_);
        }
        prelude.push_str("};\n");
    }
    if !state.use_resources.is_empty() {
        prelude.push_str("use crate::resources::{");
        for (n, type_) in state
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
            .enumerate()
        {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(type_);
        }
        prelude.push_str("};\n");
    }
    prelude.push_str("use serde::{Deserialize, Serialize};\n");
    prelude.push('\n');
    prelude
}

#[tracing::instrument(skip_all)]
pub fn gen_generated_schemas(
    out: &mut String,
    state: &mut FileGenerator,
    meta: &Metadata,
    shared_objects: &mut BTreeSet<FileGenerator>,
) {
    while let Some(schema_name) =
        state.generated_schemas.iter().find_map(|(k, &v)| if !v { Some(k) } else { None }).cloned()
    {
        let struct_name = meta.schema_to_rust_type(&schema_name);
        trace!("struct {} {{...}} ({})", struct_name, schema_name);
        out.push('\n');
        out.push_str("#[derive(Clone, Debug, Default, Deserialize, Serialize)]\n");
        out.push_str("pub struct ");
        out.push_str(&struct_name);
        out.push_str(" {\n");

        let schema = meta.spec.get_schema_unwrapped(&schema_name).as_item().expect("Expected item");
        let obj_type = as_object_type(schema).expect("Expected object type schema");
        for (key, field) in &obj_type.properties {
            let required = obj_type.required.contains(key);
            out.push('\n');
            out.push_str(&gen_field(
                state,
                meta,
                &schema_name,
                key,
                field,
                required,
                false,
                shared_objects,
            ));
        }
        out.push_str("}\n");

        // Set the schema to generated
        *state.generated_schemas.entry(schema_name).or_default() = true;
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_multitype_params(
    parent_struct_rust_type: &str,
    param_name: &str,
    param: &Parameter,
    initializers: &mut Vec<(String, String, bool)>,
    required: bool,
    state: &mut FileGenerator,
    meta: &Metadata,
    out: &mut String,
) {
    let member_schema = match &param.parameter_data_ref().format {
        ParameterSchemaOrContent::Schema(ReferenceOr::Item(s)) => s,
        _ => panic!("Expected schema content"),
    };
    match gen_member_variable_string(member_schema) {
        Ok(type_) => {
            initializers.push((param_name.into(), type_.clone(), required));
            write_out_field(out, param_name, &type_, required);
        }
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
                initializers.push((param_name.into(), new_type_name.clone(), required));
                write_out_field(out, param_name, &new_type_name, required);
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
            initializers.push((param_name.into(), new_type_name.clone(), required));
            write_out_field(out, param_name, &new_type_name, required);
        }
        _ => {
            panic!("Don't recognize this: {:#?}", member_schema);
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_inferred_params(
    out: &mut String,
    state: &mut FileGenerator,
    meta: &Metadata,
    shared_objects: &mut BTreeSet<FileGenerator>,
) {
    let object = state.name.clone();
    let id_type = meta.schema_to_id_type(&object);
    let struct_name = meta.schema_to_rust_type(&object);

    for (_, params) in state.inferred_parameters.clone() {
        let params_schema = params.rust_type.to_snake_case();

        // Derive Default when no param is required
        let can_derive_default =
            params.parameters.iter().all(|param| !param.parameter_data_ref().required);

        out.push('\n');
        out.push_str(&format!("/// The parameters for `{}::{}`.\n", struct_name, params.method));

        if can_derive_default {
            out.push_str("#[derive(Clone, Debug, Serialize, Default)]\n");
        } else {
            out.push_str("#[derive(Clone, Debug, Serialize)]\n");
        }

        trace!("struct {} {{...}}", &params.rust_type);

        out.push_str("pub struct ");
        out.push_str(&params.rust_type);
        out.push_str("<'a> {\n");
        let mut initializers: Vec<(String, String, bool)> = Vec::new();
        for param in params.parameters.iter().filter(|p| matches!(p, Parameter::Query { .. })) {
            let param_name = param.parameter_data_ref().name.as_str();
            let param_rename = match param_name {
                "type" => "type_",
                other => other,
            };
            let print_doc = |out: &mut String| {
                out.push('\n');
                if let Some(doc) = &param.parameter_data_ref().description {
                    print_doc_comment(out, doc, 1);
                }
                if param_rename != param_name {
                    write_serde_rename(out, param_name);
                }
            };
            let required = param.parameter_data_ref().required;
            match param_name {
                // TODO: Handle these unusual params
                "bank_account" | "usage" => continue,
                "destination" | "card" => {
                    print_doc(out);
                    gen_multitype_params(
                        &params.rust_type,
                        param_name,
                        param,
                        &mut initializers,
                        required,
                        state,
                        meta,
                        out,
                    );
                }
                "product" => {
                    print_doc(out);
                    initializers.push((
                        "product".into(),
                        "IdOrCreate<'a, CreateProduct<'a>>".into(),
                        required,
                    ));
                    state.use_params.insert("IdOrCreate");
                    state.use_resources.insert("CreateProduct".to_owned());
                    write_out_field(out, "product", "IdOrCreate<'a, CreateProduct<'a>>", required);
                }
                "metadata" => {
                    print_doc(out);
                    initializers.push(("metadata".into(), "Metadata".into(), required));
                    state.use_params.insert("Metadata");
                    write_out_field(out, "metadata", "Metadata", required);
                }
                "expand" => {
                    print_doc(out);
                    initializers.push(("expand".into(), "&'a [&'a str]".into(), false));
                    state.use_params.insert("Expand");
                    out.push_str("    #[serde(skip_serializing_if = \"Expand::is_empty\")]\n");
                    out.push_str("    pub expand: &'a [&'a str],\n");
                }
                "limit" => {
                    print_doc(out);
                    initializers.push(("limit".into(), "u64".into(), false));
                    write_out_field(out, "limit", "u64", required);
                }
                "ending_before" => {
                    print_doc(out);
                    let cursor_type =
                        id_type.as_ref().map(|(x, _)| x.as_str()).unwrap_or("&'a str");
                    initializers.push(("ending_before".into(), cursor_type.into(), false));
                    if required {
                        panic!("unexpected \"required\" `ending_before` parameter");
                    } else {
                        write_out_field(out, "ending_before", cursor_type, false);
                    }
                }
                "starting_after" => {
                    print_doc(out);
                    let cursor_type =
                        id_type.as_ref().map(|(x, _)| x.as_str()).unwrap_or("&'a str");
                    initializers.push(("starting_after".into(), cursor_type.into(), false));
                    if required {
                        panic!("unexpected \"required\" `starting_after` parameter");
                    } else {
                        write_out_field(out, "starting_after", cursor_type, false);
                    }
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
                        print_doc(out);
                        initializers.push((param_rename.into(), rust_type.to_string(), required));
                        state.add_use(use_path);
                        if rust_type.starts_with("Option<") {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                        }
                        out.push_str("    pub ");
                        out.push_str(param_rename);
                        out.push_str(": ");
                        out.push_str(rust_type);
                        out.push_str(",\n");
                    } else if meta.schema_to_id_type(param_name).is_some()
                        && is_string_schema
                        && param_name != "tax_id"
                    {
                        let (id_type, _) = meta.schema_to_id_type(param_name).unwrap();
                        print_doc(out);
                        initializers.push((param_name.into(), id_type.clone(), required));
                        state.use_ids.insert(id_type.clone());
                        write_out_field(out, param_rename, &id_type, required);
                    } else if matches!(kind, Some(SchemaKind::Type(Type::Boolean { .. }))) {
                        print_doc(out);
                        initializers.push((param_rename.into(), "bool".into(), false));
                        write_out_field(out, param_rename, "bool", required);
                    } else if let Some(SchemaKind::Type(Type::Integer(int_type))) = kind {
                        let rust_type = infer_integer_type(state, param_name, &int_type.format);
                        print_doc(out);
                        initializers.push((param_rename.into(), rust_type.clone(), required));
                        write_out_field(out, param_rename, &rust_type, required);
                    } else if matches!(kind, Some(SchemaKind::Type(Type::Number(_)))) {
                        print_doc(out);
                        initializers.push((param_rename.into(), "f64".into(), required));
                        write_out_field(out, param_rename, "f64", required);
                    } else if schema.as_item().and_then(as_any_of_first_item_title)
                        == Some("range_query_specs")
                    {
                        print_doc(out);
                        initializers.push((
                            param_rename.into(),
                            "RangeQuery<Timestamp>".into(),
                            required,
                        ));
                        state.use_params.insert("RangeQuery");
                        state.use_params.insert("Timestamp");
                        write_out_field(out, param_rename, "RangeQuery<Timestamp>", required);
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

                        print_doc(out);
                        initializers.push((param_rename.into(), enum_name.clone(), required));
                        write_out_field(out, param_rename, &enum_name, required);
                    } else if (param_name == "currency" || param_name.ends_with("_currency"))
                        && is_string_schema
                    {
                        print_doc(out);
                        initializers.push((param_rename.into(), "Currency".into(), required));
                        state.use_resources.insert("Currency".into());
                        write_out_field(out, param_rename, "Currency", required);
                    } else if is_string_schema {
                        print_doc(out);
                        initializers.push((param_rename.into(), "&'a str".into(), required));
                        write_out_field(out, param_rename, "&'a str", required);
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
                        initializers.push((param_rename.into(), rust_type.clone(), required));

                        print_doc(out);
                        if !required {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                        }
                        out.push_str("    pub ");
                        out.push_str(param_rename);
                        out.push_str(": ");
                        out.push_str(&rust_type);
                        out.push_str(",\n");
                    } else if required {
                        panic!("error: skipped required parameter: {} => {:?}", param_name, schema);
                    } else {
                        tracing::warn!("skipping optional parameter: {}", param_name);
                    }
                }
            }
        }
        out.push_str("}\n");
        out.push('\n');
        out.push_str("impl<'a> ");
        out.push_str(&params.rust_type);
        out.push_str("<'a> {\n");
        out.push_str("    pub fn new(");
        let mut required_count = 0;
        for (name, type_, required) in &initializers {
            if *required {
                if required_count > 0 {
                    out.push_str(", ");
                }
                out.push_str(name);
                out.push_str(": ");
                out.push_str(type_);
                required_count += 1;
            }
        }
        out.push_str(") -> Self {\n");
        out.push_str("        ");
        out.push_str(&params.rust_type);
        out.push_str(" {\n");
        for (name, _, required) in &initializers {
            out.push_str("            ");
            out.push_str(name);
            if *required {
                out.push_str(",\n");
            } else {
                out.push_str(": Default::default(),\n");
            }
        }
        out.push_str("        }\n");
        out.push_str("    }\n");
        out.push_str("}\n");

        // we implement paginate on lists that have an Id
        if let ("list", Some(_)) = (params.method.as_str(), &id_type) {
            state.use_params.insert("Paginable");

            out.push_str("impl Paginable for ");
            out.push_str(&params.rust_type);
            out.push_str("<'_> {\n");
            out.push_str("    type O = ");
            out.push_str(&struct_name);
            out.push_str(";\n");
            out.push_str(
                "    fn set_last(&mut self, item: Self::O) {
                self.starting_after = Some(item.id());
            }",
            );
            out.push_str("}");
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_inferred_structs(
    out: &mut String,
    state: &mut FileGenerator,
    meta: &Metadata,
    shared_objects: &mut BTreeSet<FileGenerator>,
) {
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
            }

            let obj = match struct_.schema.schema_kind {
                SchemaKind::Type(Type::Object(o)) => o,
                _ => {
                    // TODO: Handle these objects
                    tracing::warn!("{} has no properties ({:#?})", struct_name, struct_.schema);
                    continue;
                }
            };

            trace!("struct {} {{...}}", &struct_name.to_camel_case());

            out.push('\n');
            out.push_str("#[derive(Clone, Debug, Default, Deserialize, Serialize)]\n");
            out.push_str("pub struct ");
            out.push_str(&struct_name.to_camel_case());
            out.push_str(" {\n");

            for (key, field) in &obj.properties {
                let required = obj.required.contains(key);
                out.push('\n');
                out.push_str(&gen_field(
                    state,
                    meta,
                    &struct_name.to_snake_case(),
                    key,
                    field,
                    required,
                    false,
                    shared_objects,
                ));
            }
            out.push_str("}\n");
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn gen_unions(out: &mut String, unions: &BTreeMap<String, InferredUnion>, meta: &Metadata) {
    for (union_name, union_) in unions {
        trace!("union {} {{ ... }}", union_name);

        out.push('\n');
        out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
        out.push_str("#[serde(untagged, rename_all = \"snake_case\")]\n");
        out.push_str("pub enum ");
        out.push_str(&union_name.to_camel_case());
        out.push_str(" {\n");
        for variant_schema in &union_.schema_variants {
            let schema =
                meta.spec.get_schema_unwrapped(variant_schema).as_item().expect("Expected an item");
            let object_name = as_object_enum_name(schema)
                .unwrap_or_else(|| schema.schema_data.title.clone().unwrap());
            let variant_name = meta.schema_to_rust_type(&object_name);
            let type_name = meta.schema_to_rust_type(variant_schema);
            if variant_to_serde_snake_case(&variant_name) != object_name {
                write_serde_rename(out, &object_name);
            }
            out.push_str("    ");
            out.push_str(&variant_name);
            out.push('(');
            out.push_str(&type_name);
            out.push_str("),\n");
        }
        out.push_str("}\n");

        if let Some(first) = union_
            .schema_variants
            .iter()
            .filter_map(|var| match var.trim() {
                "" => None,
                n => Some(n),
            })
            .map(|s| gen_variant_name(s, meta))
            .next()
        {
            let struct_name = union_name.to_camel_case();
            writedoc!(
                out,
                r"
                impl std::default::Default for {struct_name} {{
                    fn default() -> Self {{
                        Self::{first}(Default::default())
                    }}
                }}
                "
            )
            .unwrap();
        }
    }
}

/// This code is taken from serde RenameRule::apply_to_variant
/// It differs in some cases from heck, so we need to make sure we
/// do exactly the same when figuring out whether we need a serde(rename)
/// e.g. heck_snake(Self_) = self
/// serde_snake(Self_) = self_
pub fn variant_to_serde_snake_case(variant: &str) -> String {
    let mut snake = String::new();
    for (i, ch) in variant.char_indices() {
        if i > 0 && ch.is_uppercase() {
            snake.push('_');
        }
        snake.push(ch.to_ascii_lowercase());
    }
    snake
}

#[tracing::instrument(skip_all)]
pub fn gen_variant_name(wire_name: &str, meta: &Metadata) -> String {
    match wire_name {
        "*" => "All".to_string(),
        "self" => "Self_".to_string(),
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
pub fn gen_enums(out: &mut String, enums: &BTreeMap<String, InferredEnum>, meta: &Metadata) {
    for (enum_name, enum_) in enums {
        trace!("enum {} {{ ... }}", enum_name);

        out.push('\n');
        out.push_str(&format!(
            "/// An enum representing the possible values of an `{}`'s `{}` field.\n",
            enum_.parent, enum_.field
        ));
        out.push_str("#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]\n");
        out.push_str("#[serde(rename_all = \"snake_case\")]\n");
        out.push_str("pub enum ");
        out.push_str(enum_name);
        out.push_str(" {\n");
        for wire_name in &enum_.options {
            if wire_name.trim().is_empty() {
                continue;
            }
            let variant_name = gen_variant_name(wire_name.as_str(), meta);
            if variant_name.trim().is_empty() {
                panic!("unhandled enum variant: {:?}", wire_name)
            }
            if &variant_to_serde_snake_case(&variant_name) != wire_name {
                write_serde_rename(out, wire_name);
            }
            out.push_str("    ");
            out.push_str(&variant_name);
            out.push_str(",\n");
        }
        out.push_str("}\n");
        out.push('\n');
        out.push_str("impl ");
        out.push_str(enum_name);
        out.push_str(" {\n");
        out.push_str("    pub fn as_str(self) -> &'static str {\n");
        out.push_str("        match self {\n");
        for wire_name in &enum_.options {
            if wire_name.trim().is_empty() {
                continue;
            }
            let variant_name = gen_variant_name(wire_name.as_str(), meta);
            out.push_str("            ");
            out.push_str(enum_name);
            out.push_str("::");
            out.push_str(&variant_name);
            out.push_str(" => ");
            out.push_str(&format!("{:?}", wire_name));
            out.push_str(",\n");
        }
        out.push_str("        }\n");
        out.push_str("    }\n");
        out.push_str("}\n");
        out.push('\n');
        writedoc!(
            out,
            r"
        impl AsRef<str> for {enum_name} {{
            fn as_ref(&self) -> &str {{
                self.as_str()
            }}
        }}
        
        impl std::fmt::Display for {enum_name} {{
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{
                self.as_str().fmt(f)
            }}
        }}
        "
        )
        .unwrap();
        if let Some(first) = enum_
            .options
            .iter()
            .filter_map(|var| match var.trim() {
                "" => None,
                n => Some(n),
            })
            .map(|s| gen_variant_name(s, meta))
            .next()
        {
            out.push_str("impl std::default::Default for ");
            out.push_str(enum_name);
            out.push_str(" {\n");
            out.push_str("    fn default() -> Self {\n");
            out.push_str(&format!("        Self::{}\n", first));
            out.push_str("    }\n");
            out.push_str("}\n");
        }
    }
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
pub fn gen_objects(out: &mut String, objects: &BTreeMap<String, InferredObject>) {
    let mut generated_objects = objects.clone();
    while !generated_objects.is_empty() {
        let key_str: String;
        let value_obj: InferredObject;
        {
            let (key, value) = generated_objects.iter().next().unwrap();
            trace!("object: {} -- {:#?}", key, value);
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
        out.push('\n');
        print_doc_from_schema(out, &schema, 0);

        match schema.schema_kind {
            SchemaKind::Type(Type::Object(obj)) => {
                trace!("struct {} {{...}}", key_str);

                out.push_str("#[derive(Clone, Debug, Default, Deserialize, Serialize)]\n");
                out.push_str(&format!("pub struct {} {{\n", key_str));
                let props = obj.properties.iter().flat_map(|(name, schema_or)| match schema_or {
                    ReferenceOr::Reference { .. } => None,
                    ReferenceOr::Item(schema) => Some((name, schema)),
                });
                for (member_name, member_schema) in props {
                    let is_required = obj.required.contains(member_name);
                    print_doc_from_schema(out, member_schema, 1);
                    match gen_member_variable_string(member_schema) {
                        Ok(normal_var) => {
                            write_out_field(out, member_name, &normal_var, is_required)
                        }
                        Err(TypeError::IsObject) => {
                            let rust_type = member_name.to_camel_case();
                            write_out_field(out, member_name, &rust_type, is_required);
                            let new_params = InferredObject {
                                rust_type: rust_type.clone(),
                                schema: *member_schema.clone(),
                            };
                            generated_objects.insert(rust_type, new_params);
                        }
                        _ => {
                            panic!("Unhandled case, inspect: {:#?}", member_schema);
                        }
                    }
                }
                out.push_str("}\n");
            }
            SchemaKind::AnyOf { any_of } => {
                out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
                out.push_str("#[serde(untagged, rename_all = \"snake_case\")]\n");
                out.push_str(&format!("pub enum {} {{\n", key_str));

                let mut variants = HashMap::new();

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
                            let count = variants.entry(type_name.clone()).or_insert(0);
                            let suffix =
                                if *count == 0 { "".to_string() } else { count.to_string() };
                            *count += 1;

                            format!("{}{}", type_name, suffix)
                        });

                    out.push_str(&format!("    pub {}({}),\n", variant_name, type_name));
                }
                out.push_str("}\n");
            }
            other => panic!("Expected an object here got: {:?}", other),
        }
        generated_objects.remove(&key_str);
    }
}

#[tracing::instrument(skip_all, fields(object = object, field_name = field_name))]
pub fn gen_field<T: Borrow<Schema>>(
    state: &mut FileGenerator,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &ReferenceOr<T>,
    required: bool,
    default: bool,
    shared_objects: &mut BTreeSet<FileGenerator>,
) -> String {
    trace!("gen_field: {}::{} (required: {}, default: {})", object, field_name, required, default);

    let mut out = String::new();
    if let Some(doc) = field.as_item().and_then(|s| s.borrow().schema_data.description.as_deref()) {
        print_doc_comment(&mut out, doc, 1);
    }
    let mut field_rename = field_name.to_snake_case();
    if field_rename == "type" {
        field_rename = "type_".into();
    }
    if field_rename != field_name {
        write_serde_rename(&mut out, field_name);
    }
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
    if !required {
        if rust_type == "bool" || rust_type == "Metadata" || rust_type.starts_with("List<") {
            out.push_str("    #[serde(default)]\n");
        } else if rust_type.starts_with("Option<") {
            out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
        }
    }
    out.push_str("    pub ");
    out.push_str(&field_rename);
    out.push_str(": ");
    out.push_str(&rust_type);
    out.push_str(",\n");
    out
}

#[tracing::instrument(skip_all, fields(object = object, path = path))]
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
            // for some reason, this field causes clashes, so just skip it
            // until the new codegen is ready
            if schema_name != "invoice_setting_subscription_schedule_setting" {
                state.generated_schemas.insert(schema_name.into(), false);
            }
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
        SchemaKind::Type(Type::Boolean(_)) => "bool".into(),
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
                trace!("object: {}, field_name: {}", object, field_name);
                let union_addition = format!("{field_name}_union");
                let union_schema = meta.schema_field(object, &union_addition);
                let union_name = meta.schema_to_rust_type(&union_schema);
                trace!("union_schema: {}, union_name: {}", union_schema, union_name);
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
        return if !required || is_nullable {
            "Option<Metadata>".into()
        } else {
            "Metadata".into()
        };
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
    } else if field_name == "type" && object == "event" {
        state.use_resources.insert("EventType".into());
        return if !required || is_nullable {
            "Option<EventType>".into()
        } else {
            "EventType".into()
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

    // currency_options field is represented by an optional HashMap<String, T>, where the String is the currency code in ISO 4217 format.
    if field_name == "currency_options" {
        state.use_params.insert("CurrencyMap");

        if ty.contains("CurrencyMap<") {
            return ty;
        }

        return format!("Option<CurrencyMap<{}>>", ty);
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

#[tracing::instrument(skip_all, fields(name = %state.name))]
pub fn gen_impl_requests(
    state: &mut FileGenerator,
    meta: &Metadata,
    object_id: Option<&str>,
) -> Option<String> {
    let name = state.name.clone();
    let object = &name;
    let requests = meta.requests.get(object)?;
    let rust_struct = meta.schema_to_rust_type(object);
    trace!("impl {} {{ ... }}", rust_struct);

    let mut methods = BTreeMap::new();

    for path in requests {
        // Unwrapped is safe here to avoid dealing with an `Option` since these paths come
        // from the spec already
        let request = meta
            .spec
            .get_request_unwrapped(*path)
            .as_item()
            .expect("Expected item, not path reference");
        let segments = path.trim_start_matches("/v1/").split('/').collect::<Vec<_>>();

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
                && !methods.contains_key(&MethodTypes::List)
            {
                let params_name = if rust_struct.ends_with('y') {
                    format!("List{}ies", &rust_struct[0..rust_struct.len() - 1])
                } else {
                    format!("List{}s", rust_struct)
                };
                let params = InferredParams {
                    method: "list".into(),
                    rust_type: params_name.clone(),
                    parameters: non_path_ref_params(get_request),
                };
                state.inferred_parameters.insert(params_name.to_snake_case(), params);
                state.use_params.insert("List");

                let mut out = String::new();
                out.push('\n');
                print_doc_comment(&mut out, doc_comment, 1);

                let query_path = segments.join("/");
                writedoc!(&mut out, r#"
                    pub fn list(client: &Client, params: &{params_name}<'_>) -> Response<List<{rust_struct}>> {{
                       client.get_query("/{query_path}", params)
                    }}
                "#).unwrap();
                methods.insert(MethodTypes::List, out);
            } else if segments.len() == 2 && !methods.contains_key(&MethodTypes::Retrieve) {
                let id_param = match get_id_param(&get_request.parameters) {
                    Some(p) => p,
                    None => continue,
                };
                let expand_param = find_param_by_name(get_request, "expand");
                if let Some(id_type) = &object_id {
                    assert!(id_param.parameter_data_ref().required);
                    assert!(matches!(id_param, Parameter::Path { style: PathStyle::Simple, .. }));

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, doc_comment, 1);
                    out.push_str("    pub fn retrieve(client: &Client, id: &");
                    out.push_str(id_type);
                    if let Some(param) = expand_param {
                        state.use_params.insert("Expand");
                        assert!(matches!(param, Parameter::Query { .. }));
                        out.push_str(", expand: &[&str]) -> Response<");
                        out.push_str(&rust_struct);
                        out.push_str("> {\n");
                        out.push_str("        client.get_query(");
                        out.push_str(&format!("&format!(\"/{}/{{}}\", id)", segments[0]));
                        out.push_str(", Expand { expand })\n");
                    } else {
                        out.push_str(") -> Response<");
                        out.push_str(&rust_struct);
                        out.push_str("> {\n");
                        out.push_str("        client.get(/");
                        out.push_str(&format!("&format!(\"/{}/{{}}\", id)", segments[0]));
                        out.push_str(")\n");
                    }
                    out.push_str("    }");
                    methods.insert(MethodTypes::Retrieve, out);
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

            if !methods.contains_key(&MethodTypes::Create) && parameter_count == 0 && create {
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

                let mut out = String::new();
                out.push('\n');
                print_doc_comment(&mut out, doc_comment, 1);
                out.push_str("    pub fn create(client: &Client, params: ");
                out.push_str(&params_name);
                out.push_str("<'_>) -> Response<");
                out.push_str(&return_type);
                out.push_str("> {\n");
                out.push_str("        #[allow(clippy::needless_borrows_for_generic_args)]\n");
                out.push_str("        client.post_form(\"/");
                out.push_str(&segments.join("/"));
                out.push_str("\", &params)\n");
                out.push_str("    }");
                methods.insert(MethodTypes::Create, out);
            } else if !methods.contains_key(&MethodTypes::Update) && parameter_count == 1 && update
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

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, doc_comment, 1);
                    out.push_str("    pub fn update(client: &Client, id: &");
                    out.push_str(id_type);
                    out.push_str(", params: ");
                    out.push_str(&params_name);
                    out.push_str("<'_>) -> Response<");
                    out.push_str(&return_type);
                    out.push_str("> {\n");
                    out.push_str("        #[allow(clippy::needless_borrows_for_generic_args)]\n");
                    out.push_str("        client.post_form(");
                    out.push_str(&format!("&format!(\"/{}/{{}}\", id)", segments[0]));
                    out.push_str(", &params)\n");
                    out.push_str("    }");
                    methods.insert(MethodTypes::Update, out);
                }
            } else {
                tracing::warn!(
                    "unhandled {} for {rust_struct}: POST {path} (already have {:?})",
                    match (create, update) {
                        (true, _) => "CREATE",
                        (_, true) => "UPDATE",
                        _ => "UNKNOWN",
                    },
                    methods.keys()
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
            if segments.len() == 2 && !methods.contains_key(&MethodTypes::Delete) {
                let id_param = match get_id_param(&delete_request.parameters) {
                    Some(p) => p,
                    None => continue,
                };
                if let Some(id_type) = &object_id {
                    state.use_params.insert("Deleted");
                    assert!(id_param.parameter_data_ref().required);
                    assert!(matches!(id_param, Parameter::Path { style: PathStyle::Simple, .. }));

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, doc_comment, 1);
                    out.push_str("    pub fn delete(client: &Client, id: &");
                    out.push_str(id_type);
                    out.push_str(") -> Response<Deleted<");
                    out.push_str(id_type);
                    out.push_str(">> {\n");
                    out.push_str("        client.delete(");
                    out.push_str(&format!("&format!(\"/{}/{{}}\", id)", segments[0]));
                    out.push_str(")\n");
                    out.push_str("    }");
                    methods.insert(MethodTypes::Delete, out);
                }
            } else {
                tracing::warn!("unhandled DELETE for {rust_struct}: {path}");
            }
        }
    }

    if methods.is_empty() {
        None
    } else {
        // Add imports
        state.use_config.insert("Client");
        state.use_config.insert("Response");

        // Output the impl block
        Some(format!(
            "impl {} {{\n{}\n}}\n",
            rust_struct,
            methods.values().map(String::as_str).collect::<Vec<_>>().join("\n")
        ))
    }
}
