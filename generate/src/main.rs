use heck::{CamelCase, SnakeCase};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

fn main() {
    let raw = fs::read_to_string("generate/spec3.json").unwrap();
    let spec: Value = serde_json::from_str(&raw).unwrap();

    // Compute additional metadata from spec.
    let mut objects = BTreeSet::new();
    let mut depends: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    let mut reverse: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    let mut requests: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    let mut renames = BTreeMap::new();
    for (key, schema) in spec["components"]["schemas"].as_object().unwrap() {
        let schema_name = key.as_str();
        let fields = match schema["properties"].as_object() {
            Some(some) => some,
            None => continue,
        };
        if fields.contains_key("object") {
            objects.insert(schema_name);
        }
        for (_, field) in fields {
            if let Some(path) = field["$ref"].as_str() {
                let dep = path.trim_start_matches("#/components/schemas/");
                depends.entry(schema_name).or_default().insert(dep);
                reverse.entry(dep).or_default().insert(schema_name);
            }
            if let Some(any_of) = field["anyOf"].as_array() {
                for ty in any_of {
                    if let Some(path) = ty["$ref"].as_str() {
                        let dep = path.trim_start_matches("#/components/schemas/");
                        depends.entry(schema_name).or_default().insert(dep);
                        reverse.entry(dep).or_default().insert(schema_name);
                    }
                }
            }
        }
    }
    for (path, _) in spec["paths"].as_object().unwrap() {
        let seg = path.trim_start_matches("/v1/").split("/").into_iter().next().unwrap();
        let seg_like = &seg[0..seg.len() - 1];
        if seg.ends_with("s") && objects.contains(seg_like) {
            requests.entry(seg_like).or_default().insert(path.as_str());
        }
    }
    /*
    for (key, _) in spec["components"]["schemas"].as_object().unwrap() {
        let key = key.as_str();
        if !objects.contains(key) {
            if let Some(set) = reverse.get(key) {
                if set.len() == 1 {
                    let rev = set.iter().next().unwrap();
                    let prefix = format!("{}_", rev);
                    if key.starts_with(&prefix) {
                        renames.insert(key, key.trim_start_matches(&prefix));
                    }
                }
            }
        }
    }
    */
    renames.insert("account_business_profile", "business_profile");
    renames.insert("account_branding_settings", "branding_settings");
    renames.insert("account_card_payments_settings", "card_payments_settings");
    renames.insert("account_dashboard_settings", "dashboard_settings");
    renames.insert("account_payments_settings", "payments_settings");
    renames.insert("account_payout_settings", "payout_settings");
    renames.insert("account_tos_acceptance", "tos_acceptance");
    renames.insert("account_decline_charge_on", "decline_charge_on");
    let meta = Metadata { spec: &spec, objects, depends, reverse, requests, renames };

    // Generate files
    for object in &meta.objects {
        let out = gen_impl_object(&meta, object);

        // TODO: Finish impl by writing to file
        println!("{}", out);
        std::process::exit(0);
    }
}

struct Metadata<'a> {
    spec: &'a Value,
    /// The set of schemas which should implement `Object`.
    /// These have both an `id` property and on `object` property.
    objects: BTreeSet<&'a str>,
    /// A one to many map of schema to depended-on types.
    depends: BTreeMap<&'a str, BTreeSet<&'a str>>,
    /// A one to many map of schema to depending types.
    reverse: BTreeMap<&'a str, BTreeSet<&'a str>>,
    /// A one to many map of _objects_ to requests which should be
    /// implemented for that object.
    ///
    /// This is typically determined by the first segment in the path.
    requests: BTreeMap<&'a str, BTreeSet<&'a str>>,
    /// How a particular schema should be renamed.
    renames: BTreeMap<&'a str, &'a str>,
}

#[derive(Default)]
struct ImplMetadata {
    /// The ids that must be imported in this file.
    use_ids: BTreeSet<String>,
    /// The params that must be imported in this file.
    use_params: BTreeSet<&'static str>,
    /// The resources that must be imported in this file.
    use_resources: BTreeSet<String>,
    /// Extra (simple) enums that were / will be generated in this file.
    inferred_enums: BTreeMap<String, InferredEnum>,
    /// Extra (complex) enums that were / will be generated in this file.
    inferred_unions: BTreeMap<String, InferredUnion>,
    /// Extra structs that were / will be generated in this file.
    inferred_structs: BTreeMap<String, InferredStruct>,
    /// The schemas that were / will be generated in this file.
    generated_schemas: BTreeMap<String, bool>,
}

#[derive(Clone)]
struct InferredEnum {
    parent: String,
    field: String,
    wire_variants: Vec<String>,
}

#[derive(Clone)]
struct InferredUnion {
    field: String,
    schema_variants: Vec<String>,
}

#[derive(Clone)]
struct InferredStruct {
    field: String,
    schema: Value,
}

fn gen_impl_object(meta: &Metadata, object: &str) -> String {
    let mut out = String::new();
    let mut impl_ = ImplMetadata::default();
    let struct_name = object.to_camel_case();
    let schema = &meta.spec["components"]["schemas"][object];
    let schema_title = schema["title"].as_str().unwrap();
    let id_type = schema["x-resourceId"].as_str().map(|id| id.to_camel_case() + "Id");
    let fields = schema["properties"].as_object().unwrap();
    let object_literal = fields["object"]["enum"][0].as_str().unwrap();

    // Generate the struct type
    out.push_str("/// The resource representing a Stripe \"");
    out.push_str(schema_title);
    out.push_str("\".\n");
    if let Some(doc_url) = check_object_doc_url(object) {
        out.push_str("///\n");
        out.push_str("/// For more details see [");
        out.push_str(&doc_url);
        out.push_str("](");
        out.push_str(&doc_url);
        out.push_str(").\n");
    }
    out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
    out.push_str("pub struct ");
    out.push_str(&struct_name);
    out.push_str(" {\n");
    if let Some(id_type) = &id_type {
        impl_.use_ids.insert(id_type.clone());
        if let Some(doc) = schema["properties"]["id"]["description"].as_str() {
            print_doc_comment(&mut out, doc);
        }
        out.push_str("    id: ");
        out.push_str(&id_type);
        out.push_str(",\n");
    }
    for (key, field) in fields {
        if key == "id" {
            continue;
        }
        if key == "object" {
            continue;
        }
        out.push('\n');
        out.push_str(&gen_field(&mut impl_, meta, object, &key, &field));
    }
    out.push_str("}\n");

    // Generate an `impl Object` block
    impl_.use_params.insert("Object");
    out.push('\n');
    out.push_str("impl Object for ");
    out.push_str(&struct_name);
    out.push_str(" {\n");
    out.push_str("    type Id = ");
    if let Some(id_type) = &id_type {
        out.push_str(&id_type);
        out.push_str(";\n");
        out.push_str("    fn id(&self) -> &Self::Id {\n        &self.id\n    }\n");
    } else {
        out.push_str("();\n");
        out.push_str("    fn id(&self) -> &Self::Id {}\n");
    }
    out.push_str("    fn object(&self) -> &'static str {\n        \"");
    out.push_str(object_literal);
    out.push_str("\"\n    }\n");
    out.push_str("}\n");

    while let Some(schema_name) = impl_
        .generated_schemas
        .iter()
        .filter_map(|(k, &v)| if !v { Some(k) } else { None })
        .next()
        .cloned()
    {
        let struct_name = if let Some(rename) = meta.renames.get(&schema_name.as_str()) {
            rename.to_camel_case()
        } else {
            schema_name.to_camel_case()
        };
        out.push('\n');
        out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
        out.push_str("pub struct ");
        out.push_str(&struct_name);
        out.push_str(" {\n");
        for (key, field) in
            meta.spec["components"]["schemas"][&schema_name]["properties"].as_object().unwrap()
        {
            out.push('\n');
            out.push_str(&gen_field(&mut impl_, meta, &schema_name, &key, &field));
        }
        out.push_str("}\n");

        // Set the schema to generated
        *impl_.generated_schemas.entry(schema_name).or_default() = true;
    }

    for (struct_name, struct_) in impl_.inferred_structs.clone() {
        out.push('\n');
        out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
        out.push_str("pub struct ");
        out.push_str(&struct_name.to_camel_case());
        out.push_str(" {\n");
        for (key, field) in struct_.schema["properties"].as_object().unwrap() {
            out.push('\n');
            out.push_str(&gen_field(&mut impl_, meta, &struct_name.to_snake_case(), &key, &field));
        }
        out.push_str("}\n");
    }

    for (union_name, union_) in impl_.inferred_unions.clone() {
        out.push('\n');
        out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
        out.push_str("#[serde(tag = \"object\", rename_all = \"snake_case\")]\n");
        out.push_str("pub enum ");
        out.push_str(&union_name.to_camel_case());
        out.push_str(" {\n");
        for variant in &union_.schema_variants {
            let object_name = meta.spec["components"]["schemas"][&variant]["properties"]["object"]
                ["enum"][0]
                .as_str()
                .unwrap();
            let variant_name = object_name.to_camel_case();
            let type_name = if let Some(rename) = meta.renames.get(variant.as_str()) {
                rename.to_camel_case()
            } else {
                variant.to_camel_case()
            };
            if variant_name.to_snake_case() != object_name {
                out.push_str("    #[serde(rename = \"");
                out.push_str(object_name);
                out.push_str("\")]\n");
            }
            out.push_str("    ");
            out.push_str(&variant_name);
            out.push_str("(");
            out.push_str(&type_name);
            out.push_str("),\n");
        }
        out.push_str("}\n");
    }

    for (enum_name, enum_) in impl_.inferred_enums.clone() {
        out.push('\n');
        out.push_str(&format!("/// An enum representing the possible values of an `{}`'s `{}` field.\n", enum_.parent, enum_.field));
        out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
        out.push_str("#[serde(rename_all = \"snake_case\")]\n");
        out.push_str("pub enum ");
        out.push_str(&enum_name.to_camel_case());
        out.push_str(" {\n");
        for wire_name in &enum_.wire_variants {
            let variant_name = wire_name.to_camel_case();
            if &variant_name.to_snake_case() != wire_name {
                out.push_str("    #[serde(rename = \"");
                out.push_str(wire_name);
                out.push_str("\")]\n");
            }
            out.push_str("    ");
            out.push_str(&variant_name);
            out.push_str(",\n");
        }
        out.push_str("}\n");
    }

    let mut prelude = String::new();
    if impl_.use_ids.len() > 0 {
        prelude.push_str("use crate::ids::{");
        for (n, id) in impl_.use_ids.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(&id);
        }
        prelude.push_str("};\n");
    }
    if impl_.use_params.len() > 0 {
        prelude.push_str("use crate::params::{");
        for (n, param) in impl_.use_params.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(&param);
        }
        prelude.push_str("};\n");
    }
    if impl_.use_resources.len() > 0 {
        prelude.push_str("use crate::resources::{");
        for (n, resource) in impl_.use_resources.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(&resource);
        }
        prelude.push_str("};\n");
    }
    prelude.push_str("use serde_derive::{Deserialize, Serialize};\n");
    prelude.push('\n');

    // Done
    prelude + &out
}

fn gen_field(
    impl_: &mut ImplMetadata,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &Value,
) -> String {
    let mut out = String::new();
    if let Some(doc) = field["description"].as_str() {
        print_doc_comment(&mut out, doc);
    }
    let mut field_rename = field_name.to_snake_case();
    if field_rename == "type" {
        field_rename = "type_".into();
    }
    if &field_rename != field_name {
        out.push_str("    #[serde(rename = \"");
        out.push_str(field_name);
        out.push_str("\")]\n");
    }
    out.push_str("    ");
    out.push_str(&field_rename);
    out.push_str(": ");
    out.push_str(&gen_field_rust_type(impl_, meta, object, &field_name, &field));
    out.push_str(",\n");
    out
}

fn gen_field_rust_type(
    impl_: &mut ImplMetadata,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &Value,
) -> String {
    if field_name == "metadata" {
        impl_.use_params.insert("Metadata");
        return "Metadata".into();
    } else if field_name == "currency" || field_name.ends_with("_currency") {
        impl_.use_resources.insert("Currency".into());
        if field["nullable"].as_bool() == Some(true) {
            return "Option<Currency>".into();
        } else {
            return "Currency".into();
        }
    }
    let ty = match field["type"].as_str() {
        Some("boolean") => "bool".into(),
        Some("integer") => {
            if field["format"].as_str() == Some("unix-time") {
                impl_.use_params.insert("Timestamp");
                "Timestamp".into()
            } else if field_name == "monthly_anchor" {
                "u8".into()
            } else if field_name.contains("days") {
                "u32".into()
            } else if field_name.contains("count")
                || field_name.contains("size")
                || field_name.contains("quantity")
            {
                "u64".into()
            } else {
                "i64".into()
            }
        }
        Some("string") => {
            if let Some(variants) = field["enum"].as_array() {
                let enum_name = format!("{}_{}", object, field_name).to_camel_case();
                let parent_name = if let Some(rename) = meta.renames.get(object) {
                    rename.to_camel_case()
                } else {
                    object.to_camel_case()
                };
                let enum_ = InferredEnum {
                    parent: parent_name,
                    field: field_name.into(),
                    wire_variants: variants
                        .into_iter()
                        .map(|x| x.as_str().unwrap().into())
                        .collect(),
                };
                impl_.inferred_enums.insert(enum_name.clone(), enum_);
                enum_name
            } else {
                "String".into()
            }
        }
        Some("array") => {
            let element = &field["items"];
            let element_type = gen_field_rust_type(impl_, meta, object, field_name, element);
            format!("Vec<{}>", element_type)
        }
        Some("object") => {
            if field["properties"]["object"]["enum"][0].as_str() == Some("list") {
                impl_.use_params.insert("List");
                let element = &field["properties"]["data"]["items"];
                let element_field_name = if field_name.ends_with("s") {
                    &field_name[0..field_name.len() - 1]
                } else {
                    field_name
                };
                let element_type =
                    gen_field_rust_type(impl_, meta, object, element_field_name, element);
                format!("List<{}>", element_type)
            } else {
                let struct_name = format!("{}_{}", object, field_name).to_camel_case();
                let struct_ = InferredStruct { field: field_name.into(), schema: field.clone() };
                impl_.inferred_structs.insert(struct_name.clone(), struct_);
                struct_name
            }
        }
        _ => {
            if let Some(path) = field["$ref"].as_str() {
                let schema_name = path.trim_start_matches("#/components/schemas/");
                let type_name = if let Some(rename) = meta.renames.get(schema_name) {
                    rename.to_camel_case()
                } else {
                    schema_name.to_camel_case()
                };
                if meta.objects.contains(schema_name)
                    || meta.reverse.get(schema_name).map(|x| x.len()).unwrap_or(0) > 1
                {
                    impl_.use_resources.insert(type_name.clone());
                } else if !impl_.generated_schemas.contains_key(schema_name) {
                    impl_.generated_schemas.insert(schema_name.into(), false);
                }
                type_name
            } else if let Some(any_of) = field["anyOf"].as_array().or(field["oneOf"].as_array()) {
                if any_of.len() == 1 {
                    gen_field_rust_type(impl_, meta, object, field_name, &any_of[0])
                } else if field["x-expansionResources"].is_object() {
                    let ty_ = gen_field_rust_type(
                        impl_,
                        meta,
                        object,
                        field_name,
                        &field["x-expansionResources"],
                    );
                    impl_.use_params.insert("Expandable");
                    format!("Expandable<{}>", ty_)
                } else {
                    let union_name = format!("{}", field_name).to_camel_case();
                    let union_ = InferredUnion {
                        field: field_name.into(),
                        schema_variants: any_of
                            .into_iter()
                            .map(|x| {
                                let schema_name = x["$ref"]
                                    .as_str()
                                    .unwrap()
                                    .trim_start_matches("#/components/schemas/");
                                let type_name = if let Some(rename) = meta.renames.get(schema_name)
                                {
                                    rename.to_camel_case()
                                } else {
                                    schema_name.to_camel_case()
                                };
                                if meta.objects.contains(schema_name)
                                    || meta.reverse.get(schema_name).map(|x| x.len()).unwrap_or(0)
                                        > 1
                                {
                                    impl_.use_resources.insert(type_name.clone());
                                } else if !impl_.generated_schemas.contains_key(schema_name) {
                                    impl_.generated_schemas.insert(schema_name.into(), false);
                                }
                                schema_name.into()
                            })
                            .collect(),
                    };
                    impl_.inferred_unions.insert(union_name.clone(), union_);
                    union_name
                }
            } else {
                eprintln!("{}.{}: {}\n", object, field_name, field);
                unimplemented!()
            }
        }
    };
    if field["nullable"].as_bool() == Some(true) {
        format!("Option<{}>", ty)
    } else {
        ty
    }
}

fn print_doc_comment(out: &mut String, comment: &str) {
    let mut doc_parts = comment.splitn(2, ". ");
    let head = doc_parts.next().unwrap();
    out.push_str("    /// ");
    out.push_str(head);
    if !head.ends_with(".") {
        out.push('.');
    }
    out.push('\n');
    if let Some(tail) = doc_parts.next() {
        out.push_str("    ///\n");
        for part in tail.split(". ") {
            out.push_str("    /// ");
            out.push_str(part);
            if !part.ends_with(".") {
                out.push('.');
            }
            out.push('\n');
        }
    }
}

fn check_object_doc_url(object: &str) -> Option<String> {
    let doc_url = format!("https://stripe.com/docs/api/{}s/object", object);
    if let Ok(mut resp) = reqwest::get(&doc_url) {
        if resp.status().is_success() {
            let text = resp.text().unwrap();
            if text.contains("<title>Stripe API Reference") && text.contains("object</title>") {
                return Some(doc_url);
            } else {
                panic!("fatal: documentation response didn't match the expected format.");
            }
        }
    }
    eprintln!("warning: could not determine doc_url for object `{}`", object);
    None
}
