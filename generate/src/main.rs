use heck::{CamelCase, SnakeCase};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

fn main() {
    let raw = fs::read_to_string("spec3.json").unwrap();
    let spec: Value = serde_json::from_str(&raw).unwrap();

    // Compute additional metadata from spec.
    let mut objects = BTreeSet::new();
    let mut depends: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    let mut reverse: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    let mut requests: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    for (key, schema) in spec["components"]["schemas"].as_object().unwrap() {
        let schema_name = key.as_str();
        let fields = match schema["properties"].as_object() {
            Some(some) => some,
            None => continue,
        };
        if fields.contains_key("object") {
            objects.insert(schema_name);
        }
        for (field_name, field) in fields {
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
    for (path, request) in spec["paths"].as_object().unwrap() {
        let seg = path.trim_start_matches("/v1/").split("/").into_iter().next().unwrap();
        let seg_like = &seg[0..seg.len() - 1];
        if seg.ends_with("s") && objects.contains(seg_like) {
            requests.entry(seg_like).or_default().insert(path.as_str());
        }
    }
    let meta = Metadata { spec: &spec, objects, depends, reverse, requests };

    // Generate files
    for object in &meta.objects {
        let (impl_out, impl_meta) = gen_impl_object(&meta, object);

        // TODO: Finish impl by writing to file
        println!("{}", impl_out);
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
}

#[derive(Default)]
struct ImplMetadata {
    /// The ids that must be imported in this file.
    use_ids: BTreeSet<String>,
    /// The params that must be imported in this file.
    use_params: BTreeSet<String>,
    /// The resources that must be imported in this file.
    use_resources: BTreeSet<String>,
    /// Extra enums that were / will be generated in this file.
    inferred_enums: BTreeMap<String, InferredEnum>,
    /// Extra structs that were / will be generated in this file.
    inferred_structs: BTreeMap<String, InferredStruct>,
    /// The schemas that were / will be generated in this file.
    inline_schemas: BTreeSet<String>,
}

struct InferredEnum {
    description: Option<String>,
    wire_variants: Vec<String>,
}

struct InferredStruct {
    field: String,
    schema: Value,
}

fn gen_impl_object(meta: &Metadata, object: &str) -> (String, ImplMetadata) {
    let mut out = String::new();
    let mut impl_ = ImplMetadata::default();
    let schema = &meta.spec["components"]["schemas"][object];
    let schema_title = schema["title"].as_str().unwrap();
    let fields = schema["properties"].as_object().unwrap();
    let file_name = object.to_snake_case() + ".rs";
    let struct_name = object.to_camel_case();
    let object_str = fields["object"]["enum"][0].as_str().unwrap();
    let id_type = schema["x-resourceId"].as_str().map(|id| id.to_camel_case() + "Id");

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
            out.push_str("    /// ");
            out.push_str(doc);
            out.push('\n');
        }
        out.push_str("    id: ");
        out.push_str(&id_type);
        out.push_str(",\n");
    }
    for (key, field) in fields {
        if let Some(doc) = field["description"].as_str() {
            out.push_str("    /// ");
            out.push_str(doc);
            out.push('\n');
        }
        let mut field_name = key.to_snake_case();
        if field_name == "type" {
            field_name = "type_".into();
        }
        if &field_name != key {
            out.push_str("    #[serde(rename = \"");
            out.push_str(key);
            out.push_str("\")]\n");
        }
        out.push_str("    ");
        out.push_str(&field_name);
        out.push_str(": ");
        out.push_str(&gen_field_rust_type(&mut impl_, meta, object, &key, &field));
        out.push_str(",\n");
    }
    out.push_str("}\n\n");

    // Generate an `impl Object` block
    impl_.use_params.insert("Object".into());
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
    out.push_str("    fn object(&self) -> &'static str {\n        ");
    out.push_str(object_str);
    out.push_str("\n    }\n");
    out.push_str("}\n");

    // Done
    (out, impl_)
}

fn gen_field_rust_type(
    impl_: &mut ImplMetadata,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &Value,
) -> String {
    let ty = match field["type"].as_str() {
        Some("boolean") => "bool".into(),
        Some("integer") => {
            if field["format"].as_str() == Some("unix-time") {
                impl_.use_params.insert("Timestamp".into());
                "Timestamp".into()
            } else {
                unimplemented!()
            }
        }
        Some("string") => {
            if let Some(variants) = field["enum"].as_array() {
                let enum_name = format!("{}_{}", object, field_name).to_camel_case();
                let enum_ = InferredEnum {
                    description: field["description"].as_str().map(|x| x.into()),
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
        Some("object") => {
            let struct_name = format!("{}_{}", object, field_name).to_camel_case();
            let struct_ = InferredStruct { field: field_name.into(), schema: field.clone() };
            impl_.inferred_structs.insert(struct_name.clone(), struct_);
            struct_name
        }
        _ => {
            if let Some(path) = field["$ref"].as_str() {
                let schema_name = path.trim_start_matches("#/components/schemas/");
                let type_name = schema_name.to_camel_case();
                if (meta.objects.contains(schema_name)
                    || meta.reverse.get(schema_name).map(|x| x.len()).unwrap_or(0) > 1)
                {
                    impl_.use_resources.insert(type_name.clone());
                }
                type_name
            } else if let Some(any_of) = field["anyOf"].as_array() {
                if any_of.len() == 1 {
                    gen_field_rust_type(impl_, meta, object, field_name, &any_of[0])
                } else {
                    eprintln!("{}.{}: {}\n", object, field_name, field);
                    unimplemented!()
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
