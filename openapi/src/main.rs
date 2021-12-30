use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

use anyhow::{anyhow, Context, Result};
use heck::{CamelCase, SnakeCase};
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::{json, Value as Json};

mod mappings;
mod metadata;

// we use a common user agent, otherwise stripe rejects the connection
const APP_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.108 Safari/537.36";

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    let in_path = args.next().unwrap_or_else(|| "spec3.json".to_string());
    let out_path = args.next().unwrap_or_else(|| "out".to_string());

    fs::create_dir_all(&out_path).context("could not create out folder")?;

    let raw = fs::File::open(in_path).context("failed to load the specfile. does it exist?")?;
    let spec: Json = serde_json::from_reader(&raw).context("failed to read json from specfile")?;

    let id_renames = mappings::id_renames();
    let object_mappings = mappings::object_mappings();
    let field_mappings = mappings::field_mappings();
    let feature_groups = metadata::feature_groups();

    // Compute additional metadata from spec.
    let mut objects = BTreeSet::new();
    let mut dependents: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    let mut id_mappings = BTreeMap::new();
    for (key, schema) in spec["components"]["schemas"].as_object().unwrap() {
        let schema_name = key.as_str();
        let fields = match schema["properties"].as_object() {
            Some(some) => some,
            None => continue,
        };
        if fields.contains_key("object") {
            objects.insert(schema_name);
            if !schema["properties"]["id"].is_null() {
                if let Some(id_src) = schema["x-resourceId"].as_str() {
                    let id_type = if let Some(rename) = id_renames.get(&id_src) {
                        rename.to_camel_case() + "Id"
                    } else {
                        id_src.replace('.', "_").to_camel_case() + "Id"
                    };
                    id_mappings.insert(
                        schema_name.replace(".", "_").to_owned(),
                        (id_type, CopyOrClone::Clone),
                    );
                }
            }
        }
        for (_, field) in fields {
            if let Some(path) = field["$ref"].as_str() {
                let dep = path.trim_start_matches("#/components/schemas/");
                dependents.entry(dep).or_default().insert(schema_name);
            }
            if let Some(any_of) = field["anyOf"].as_array() {
                for ty in any_of {
                    if let Some(path) = ty["$ref"].as_str() {
                        let dep = path.trim_start_matches("#/components/schemas/");
                        dependents.entry(dep).or_default().insert(schema_name);
                    }
                }
            }
        }
    }

    let mut requests: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    for (path, _) in spec["paths"].as_object().unwrap() {
        let mut seg_iterator = path.trim_start_matches("/v1/").split('/').into_iter();
        let object = match (seg_iterator.next(), seg_iterator.next()) {
            // handle special case for sessions
            (Some(x), Some("sessions")) => format!("{}.session", x),
            (Some(x), _) => x.to_string(),
            _ => continue,
        };

        // This isn't documented in the API reference so let's skip it
        if object == "account" {
            continue;
        }

        let seg_like = &object[0..object.len() - 1];
        if objects.contains(object.as_str()) {
            requests.entry(object).or_default().insert(path.as_str());
        } else if object.ends_with('s') && objects.contains(seg_like) {
            requests.entry(seg_like.to_string()).or_default().insert(path.as_str());
        }
    }

    // Build context
    let meta = Metadata {
        spec: &spec,
        objects,
        dependents,
        requests,
        id_mappings,
        object_mappings,
        field_mappings,
    };

    // Generate placeholders
    {
        let mut out = String::new();
        out.push_str("use crate::ids::*;\n");
        out.push_str("use crate::params::Object;\n");
        out.push_str("use serde_derive::{Deserialize, Serialize};\n");
        for (schema, feature) in &feature_groups {
            out.push('\n');
            let (id_type, c_c) =
                meta.schema_to_id_type(schema).unwrap_or_else(|| ("()".into(), CopyOrClone::Copy));
            let struct_type = meta.schema_to_rust_type(schema);
            out.push_str(&format!("#[cfg(not(feature = \"{}\"))]\n", feature));
            out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
            out.push_str(&format!("pub struct {} {{\n", struct_type));
            out.push_str(&format!("\tpub id: {},\n", id_type));
            out.push_str("}\n\n");
            out.push_str(&format!("#[cfg(not(feature = \"{}\"))]\n", feature));
            out.push_str(&format!("impl Object for {} {{\n", struct_type));
            out.push_str(&format!("\ttype Id = {};\n", id_type));
            out.push_str(&format!(
                "\tfn id(&self) -> Self::Id {{ self.id{} }}\n",
                match c_c {
                    CopyOrClone::Clone => ".clone()",
                    CopyOrClone::Copy => "",
                }
            ));
            out.push_str(&format!("\tfn object(&self) -> &'static str {{ \"{}\" }}\n", schema));
            out.push_str("}\n");
            fs::write(format!("{}/{}", &out_path, "placeholders.rs"), out.as_bytes()).unwrap();
        }
    }

    let url_finder = UrlFinder::new()?;

    // Generate resources
    let mut shared_objects = BTreeSet::new();
    for object in &meta.objects {
        if object.starts_with("deleted_") {
            continue;
        }

        // Generate the types for the object
        fs::write(
            &format!("{}/{}.rs", &out_path, object.replace('.', "_").to_snake_case()),
            gen_impl_object(&meta, object, &url_finder, &mut shared_objects).as_bytes(),
        )
        .unwrap();
    }

    println!("Shared objects: {:#?}", shared_objects);

    let mut extra_objects = BTreeSet::new();
    for object in &shared_objects {
        if object.starts_with("deleted_") {
            continue;
        }
        // Generate the types for the object
        fs::write(
            &format!("{}/{}.rs", &out_path, object.replace('.', "_").to_snake_case()),
            gen_extra_object(&meta, object, &url_finder, &mut extra_objects).as_bytes(),
        )
        .unwrap();
    }

    println!("BTreeSet: {:#?}", extra_objects);

    Ok(())
}

#[derive(Clone, Copy)]
enum CopyOrClone {
    Copy,
    Clone,
}

struct Metadata<'a> {
    spec: &'a Json,
    /// The set of schemas which should implement `Object`.
    /// These have both an `id` property and on `object` property.
    objects: BTreeSet<&'a str>,
    /// A one to many map of schema to depending types.
    dependents: BTreeMap<&'a str, BTreeSet<&'a str>>,
    /// A map of `objects` to their rust id type
    id_mappings: BTreeMap<String, (String, CopyOrClone)>,
    /// How a particular schema should be renamed.
    object_mappings: mappings::ObjectMap,
    /// An override for the rust-type of a particular object/field pair.
    field_mappings: mappings::FieldMap,
    /// A one to many map of _objects_ to requests which should be
    /// implemented for that object.
    ///
    /// This is typically determined by the first segment in the path.
    requests: BTreeMap<String, BTreeSet<&'a str>>,
}

impl<'a> Metadata<'a> {
    fn schema_to_id_type(&self, schema: &str) -> Option<(String, CopyOrClone)> {
        let schema = schema.replace('.', "_");
        self.id_mappings.get(schema.as_str()).map(ToOwned::to_owned)
    }

    fn schema_to_rust_type(&self, schema: &str) -> String {
        let schema = schema.replace('.', "_");
        if let Some(rename) = self.object_mappings.get(schema.as_str()) {
            rename.to_camel_case()
        } else {
            schema.to_camel_case()
        }
    }

    fn field_to_rust_type(
        &self,
        schema: &str,
        field: &str,
    ) -> Option<(&'static str, &'static str)> {
        let schema = schema.replace('.', "_");
        self.field_mappings.get(&(schema.as_str(), field)).copied()
    }

    fn schema_field(&self, parent: &str, field: &str) -> String {
        let parent_type = self.schema_to_rust_type(parent);
        format!("{}_{}", parent_type, field).to_snake_case()
    }
}

#[derive(Default)]
struct Generated {
    /// The ids that must be imported in this file.
    use_ids: BTreeSet<String>,
    /// The config that must be imported in this file.
    use_config: BTreeSet<&'static str>,
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
    /// The request parameter structs that were / will be generated in this file.
    inferred_parameters: BTreeMap<String, InferredParams>,
    /// The schemas that were / will be generated in this file.
    generated_schemas: BTreeMap<String, bool>,
    /// New experimental struct that will eventually do most of the general work
    generated_objects: BTreeMap<String, InferredObject>,
}

impl Generated {
    fn insert_enum(&mut self, name: impl Into<String>, enum_: InferredEnum) {
        if let Err(other) = self.try_insert_enum(name, enum_.clone()) {
            panic!("conflicting enums are not compatible:\n\t{:?}\n\t!=\n\t{:?}", enum_, other);
        }
    }

    fn try_insert_enum(
        &mut self,
        name: impl Into<String>,
        enum_: InferredEnum,
    ) -> Result<(), &InferredEnum> {
        let name = name.into();
        let mut enum_ = enum_;
        enum_.options.sort();
        if !self.inferred_enums.contains_key(&name) {
            self.inferred_enums.insert(name, enum_);
            return Ok(());
        }
        if let Some(other) = self.inferred_enums.get(&name) {
            if enum_.options != other.options {
                return Err(other);
            }
        }
        Ok(())
    }

    fn insert_struct(&mut self, name: impl Into<String>, struct_: InferredStruct) {
        if let Err(other) = self.try_insert_struct(name, struct_.clone()) {
            panic!("conflicting structs are not compatible:\n\t{:?}\n\t!=\n\t{:?}", struct_, other);
        }
    }

    fn try_insert_struct(
        &mut self,
        name: impl Into<String>,
        struct_: InferredStruct,
    ) -> Result<(), &InferredStruct> {
        let name = name.into();
        if !self.inferred_structs.contains_key(&name) {
            self.inferred_structs.insert(name, struct_);
            return Ok(());
        }
        if let Some(other) = self.inferred_structs.get(&name) {
            let mut self_schema = struct_.schema;
            let mut other_schema = other.schema.clone();
            if let Some(x) = self_schema.as_object_mut() {
                x.remove("description");
                x.remove("title");
            }
            if let Some(x) = other_schema.as_object_mut() {
                x.remove("description");
                x.remove("title");
            }
            if self_schema != other_schema {
                return Err(other);
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct InferredEnum {
    parent: String,
    field: String,
    options: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct InferredUnion {
    field: String,
    schema_variants: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct InferredStruct {
    field: String,
    schema: Json,
}

#[derive(Clone, Debug, PartialEq)]
struct InferredParams {
    method: String,
    rust_type: String,
    parameters: Json,
}

#[derive(Clone, Debug, PartialEq)]
struct InferredObject {
    rust_type: String,
    schema: Json,
}

fn gen_struct(
    out: &mut String,
    state: &mut Generated,
    meta: &Metadata,
    object: &str,
    shared_objects: &mut BTreeSet<String>,
    url_finder: &UrlFinder,
) {
    let id_type = meta.schema_to_id_type(object);
    let struct_name = meta.schema_to_rust_type(object);
    let schema = &meta.spec["components"]["schemas"][object];
    let schema_title = schema["title"].as_str().unwrap();
    let deleted_schema = &meta.spec["components"]["schemas"][format!("deleted_{}", object)];
    let fields = match schema["properties"].as_object() {
        Some(some) => some,
        None => return (),
    };

    println!("struct {} {{...}}", struct_name);

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
    out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
    out.push_str("pub struct ");
    out.push_str(&struct_name);
    out.push_str(" {\n");
    if let Some((id_type, _)) = &id_type {
        state.use_ids.insert(id_type.clone());
        if let Some(doc) = schema["properties"]["id"]["description"].as_str() {
            print_doc_comment(out, doc, 1);
        }
        if id_type == "InvoiceId" {
            out.push_str("    #[serde(default = \"InvoiceId::none\")]");
        }
        out.push_str("    pub id: ");
        out.push_str(&id_type);
        out.push_str(",\n");
    }
    let mut did_emit_deleted = false;
    for (key, field) in fields {
        if key == "id" {
            continue;
        }
        if key == "object" {
            continue;
        }
        if !did_emit_deleted
            && !deleted_schema.is_null()
            && key.as_str().cmp(&"deleted") == std::cmp::Ordering::Greater
        {
            out.push('\n');
            out.push_str("    // Always true for a deleted object\n");
            out.push_str("    #[serde(default)]\n");
            out.push_str("    pub deleted: bool,\n");
            did_emit_deleted = true;
        }
        let required = schema["required"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|x| x.as_str()).any(|x| x == key))
            .unwrap_or(false);
        let force_optional = if deleted_schema["properties"].is_object() {
            deleted_schema["properties"][&key].is_null()
        } else {
            false
        };
        out.push('\n');
        out.push_str(&gen_field(
            state,
            meta,
            object,
            &key,
            &field,
            required && !force_optional,
            false,
            shared_objects,
        ));
    }
    out.push_str("}\n");
}

fn gen_prelude(state: &Generated, meta: &Metadata, object: &str) -> String {
    let mut prelude = String::new();
    prelude.push_str("// ======================================\n");
    prelude.push_str("// This file was automatically generated.\n");
    prelude.push_str("// ======================================\n\n");
    if !state.use_config.is_empty() {
        prelude.push_str("use crate::config::{");
        for (n, type_) in state.use_config.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(&type_);
        }
        prelude.push_str("};\n");
    }
    if !state.use_ids.is_empty() {
        prelude.push_str("use crate::ids::{");
        for (n, type_) in state.use_ids.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(&type_);
        }
        prelude.push_str("};\n");
    }
    if !state.use_params.is_empty() {
        prelude.push_str("use crate::params::{");
        for (n, type_) in state.use_params.iter().enumerate() {
            if n > 0 {
                prelude.push_str(", ");
            }
            prelude.push_str(&type_);
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
            prelude.push_str(&type_);
        }
        prelude.push_str("};\n");
    }
    prelude.push_str("use serde_derive::{Deserialize, Serialize};\n");
    prelude.push('\n');
    prelude
}

fn gen_generated_schemas(
    out: &mut String,
    state: &mut Generated,
    meta: &Metadata,
    shared_objects: &mut BTreeSet<String>,
) {
    while let Some(schema_name) =
        state.generated_schemas.iter().find_map(|(k, &v)| if !v { Some(k) } else { None }).cloned()
    {
        let struct_name = meta.schema_to_rust_type(&schema_name);
        out.push('\n');
        out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
        out.push_str("pub struct ");
        out.push_str(&struct_name);
        out.push_str(" {\n");
        for (key, field) in meta.spec["components"]["schemas"][&schema_name]["properties"]
            .as_object()
            .cloned()
            .unwrap_or_default()
        {
            let required = meta.spec["components"]["schemas"][&schema_name]["required"]
                .as_array()
                .map(|arr| arr.iter().filter_map(|x| x.as_str()).any(|x| x == key))
                .unwrap_or(false);
            out.push('\n');
            out.push_str(&gen_field(
                state,
                meta,
                &schema_name,
                &key,
                &field,
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

fn write_out_field(out: &mut String, var_name: &String, var_type: &String, required: bool) {
    if required {
        out.push_str(&format!("    pub {}: {},\n", var_name, var_type));
    } else {
        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
        out.push_str(&format!("    pub {}: Option<{}>,\n", var_name, var_type));
    }
}

fn gen_inferred_params(
    out: &mut String,
    state: &mut Generated,
    meta: &Metadata,
    object: &str,
    shared_objects: &mut BTreeSet<String>,
) {
    let id_type = meta.schema_to_id_type(object);
    let struct_name = meta.schema_to_rust_type(object);

    for (_, params) in state.inferred_parameters.clone() {
        let params_schema = params.rust_type.to_snake_case();
        let parameters = match params.parameters.as_array() {
            Some(some) => some.as_slice(),
            None => &[],
        };

        // Derive Default when no param is required
        let can_derive_default =
            parameters.iter().all(|param| param["required"].as_bool() != Some(true));

        out.push('\n');
        out.push_str(&format!("/// The parameters for `{}::{}`.\n", struct_name, params.method));

        if can_derive_default {
            out.push_str("#[derive(Clone, Debug, Serialize, Default)]\n");
        } else {
            out.push_str("#[derive(Clone, Debug, Serialize)]\n");
        }

        out.push_str("pub struct ");
        out.push_str(&params.rust_type);
        out.push_str("<'a> {\n");
        let mut initializers: Vec<(String, String, bool)> = Vec::new();
        for param in parameters {
            match param["in"].as_str() {
                Some("query") | Some("form") => (),
                _ => continue,
            }

            let param_name = param["name"].as_str().unwrap();
            let param_rename = match param_name {
                "type" => "type_",
                other => other,
            };
            let print_doc = |out: &mut String| {
                out.push('\n');
                if let Some(doc) = param["description"].as_str() {
                    print_doc_comment(out, doc, 1);
                }
                if param_rename != param_name {
                    out.push_str(&format!("    #[serde(rename = \"{}\")]\n", param_name));
                }
            };
            let required = param["required"].as_bool() == Some(true);
            match param_name {
                // TODO: Handle these unusual params
                "bank_account" | "destination" | "usage" => continue,

                "card" => {
                    print_doc(out);

                    let new_type_name: String;
                    if let Some(string_type) = gen_member_variable_string(&param["schema"]) {
                        new_type_name = string_type;
                    } else {
                        new_type_name = format!("{}CardInfo", params.rust_type);
                        let inferred_object = InferredObject {
                            rust_type: new_type_name.clone(),
                            schema: param["schema"].clone(),
                        };
                        state.generated_objects.insert(new_type_name.clone(), inferred_object);
                    }

                    initializers.push(("card".into(), new_type_name.clone(), required));
                    write_out_field(out, &"card".into(), &new_type_name, required);

                    continue;
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
                    if required {
                        out.push_str("    pub product: IdOrCreate<'a, CreateProduct<'a>>,\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str(
                            "    pub product: Option<IdOrCreate<'a, CreateProduct<'a>>>,\n",
                        );
                    }
                }
                "metadata" => {
                    print_doc(out);
                    initializers.push(("metadata".into(), "Metadata".into(), required));
                    state.use_params.insert("Metadata");
                    if required {
                        out.push_str("    pub metadata: Metadata,\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    pub metadata: Option<Metadata>,\n");
                    }
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
                    if required {
                        out.push_str("    pub limit: u64,\n");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    pub limit: Option<u64>,\n");
                    }
                }
                "ending_before" => {
                    print_doc(out);
                    let cursor_type =
                        id_type.as_ref().map(|(x, _)| x.as_str()).unwrap_or("&'a str");
                    initializers.push(("ending_before".into(), cursor_type.into(), false));
                    if required {
                        panic!("unexpected \"required\" `ending_before` parameter");
                    } else {
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    pub ending_before: Option<");
                        out.push_str(cursor_type);
                        out.push_str(">,\n");
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
                        out.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                        out.push_str("    pub starting_after: Option<");
                        out.push_str(cursor_type);
                        out.push_str(">,\n");
                    }
                }
                _ => {
                    if let Some((use_path, rust_type)) =
                        meta.field_to_rust_type(params_schema.as_str(), param_name)
                    {
                        print_doc(out);
                        initializers.push((param_rename.into(), rust_type.to_string(), required));
                        match use_path {
                            "" | "String" => (),
                            "Metadata" => {
                                state.use_params.insert("Metadata");
                            }
                            path if path.ends_with("Id") && path != "TaxId" => {
                                state.use_ids.insert(path.into());
                            }
                            path => {
                                state.use_resources.insert(path.into());
                            }
                        }
                        if rust_type.starts_with("Option<") {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                        }
                        out.push_str("    pub ");
                        out.push_str(&param_rename);
                        out.push_str(": ");
                        out.push_str(&rust_type);
                        out.push_str(",\n");
                    } else if meta.schema_to_id_type(param_name).is_some()
                        && param["schema"]["type"].as_str() == Some("string")
                        && param_name != "tax_id"
                    {
                        let (id_type, _) = meta.schema_to_id_type(param_name).unwrap();
                        print_doc(out);
                        initializers.push((param_name.into(), id_type.clone(), required));
                        state.use_ids.insert(id_type.clone());
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_name);
                            out.push_str(": ");
                            out.push_str(&id_type);
                            out.push_str(",\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_name);
                            out.push_str(": Option<");
                            out.push_str(&id_type);
                            out.push_str(">,\n");
                        }
                    } else if param["schema"]["type"].as_str() == Some("boolean") {
                        print_doc(out);
                        initializers.push((param_rename.into(), "bool".into(), false));
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": bool,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<bool>,\n");
                        }
                    } else if param["schema"]["type"].as_str() == Some("integer") {
                        let rust_type = infer_integer_type(
                            state,
                            &param_name,
                            param["schema"]["format"].as_str(),
                        );

                        print_doc(out);
                        initializers.push((param_rename.into(), rust_type.clone(), required));
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": ");
                            out.push_str(&rust_type);
                            out.push_str(",\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<");
                            out.push_str(&rust_type);
                            out.push_str(">,\n");
                        }
                    } else if param["schema"]["type"].as_str() == Some("number") {
                        print_doc(out);
                        initializers.push((param_rename.into(), "f64".into(), required));
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": f64,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<f64>,\n");
                        }
                    } else if param["schema"]["anyOf"][0]["title"].as_str()
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
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": RangeQuery<Timestamp>,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<RangeQuery<Timestamp>>,\n");
                        }
                    } else if param["schema"]["type"].as_str() == Some("string")
                        && param["schema"]["enum"].is_array()
                    {
                        let enum_schema = meta.schema_field(object, param_rename);
                        let enum_name = meta.schema_to_rust_type(&enum_schema);
                        let enum_ = InferredEnum {
                            parent: params.rust_type.clone(),
                            field: param_rename.into(),
                            options: param["schema"]["enum"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|x| x.as_str().unwrap().into())
                                .collect(),
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
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": ");
                            out.push_str(&enum_name);
                            out.push_str(",\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<");
                            out.push_str(&enum_name);
                            out.push_str(">,\n");
                        }
                    } else if (param_name == "currency" || param_name.ends_with("_currency"))
                        && param["schema"]["type"].as_str() == Some("string")
                    {
                        print_doc(out);
                        initializers.push((param_rename.into(), "Currency".into(), required));
                        state.use_resources.insert("Currency".into());
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Currency,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<Currency>,\n");
                        }
                    } else if param["schema"]["type"].as_str() == Some("string") {
                        print_doc(out);
                        initializers.push((param_rename.into(), "&'a str".into(), required));
                        if required {
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": &'a str,\n");
                        } else {
                            out.push_str(
                                "    #[serde(skip_serializing_if = \"Option::is_none\")]\n",
                            );
                            out.push_str("    pub ");
                            out.push_str(param_rename);
                            out.push_str(": Option<&'a str>,\n");
                        }
                    } else if param["schema"]["type"].as_str() == Some("object")
                        || param["schema"]["type"].as_str() == Some("array")
                        || param["schema"]["$ref"].is_string()
                        || param["schema"]["anyOf"].is_array()
                    {
                        let rust_type = gen_field_rust_type(
                            state,
                            meta,
                            &params.rust_type.to_snake_case(),
                            &param_rename,
                            &param["schema"],
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
                        panic!(
                            "error: skipped required parameter: {} => {:?}",
                            param_name, param["schema"]
                        );
                    } else {
                        eprintln!("warning: skipping optional parameter: {}", param_name);
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
                out.push_str(&name);
                out.push_str(": ");
                out.push_str(&type_);
                required_count += 1;
            }
        }
        out.push_str(") -> Self {\n");
        out.push_str("        ");
        out.push_str(&params.rust_type);
        out.push_str(" {\n");
        for (name, _, required) in &initializers {
            out.push_str("            ");
            out.push_str(&name);
            if *required {
                out.push_str(",\n");
            } else {
                out.push_str(": Default::default(),\n");
            }
        }
        out.push_str("        }\n");
        out.push_str("    }\n");
        out.push_str("}\n");
    }
}

fn gen_emitted_structs(
    out: &mut String,
    state: &mut Generated,
    meta: &Metadata,
    shared_objects: &mut BTreeSet<String>,
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
                println!("struct {} {{ ... }}", struct_name);
            }

            let fields = match struct_.schema["properties"].as_object() {
                Some(some) => some,
                None => {
                    // TODO: Handle these objects
                    // eprintln!("warning: {} has no properties ({})", struct_name, struct_.schema);
                    continue;
                }
            };
            out.push('\n');
            out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
            out.push_str("pub struct ");
            out.push_str(&struct_name.to_camel_case());
            out.push_str(" {\n");
            for (key, field) in fields {
                let required = struct_.schema["required"]
                    .as_array()
                    .map(|arr| arr.iter().filter_map(|x| x.as_str()).any(|x| x == key))
                    .unwrap_or(false);
                out.push('\n');
                out.push_str(&gen_field(
                    state,
                    meta,
                    &struct_name.to_snake_case(),
                    &key,
                    &field,
                    required,
                    false,
                    shared_objects,
                ));
            }
            out.push_str("}\n");
        }
    }
}

fn gen_unions(out: &mut String, state: &mut Generated, meta: &Metadata) {
    for (union_name, union_) in state.inferred_unions.clone() {
        println!("union {} {{ ... }}", union_name);

        out.push('\n');
        out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
        out.push_str("#[serde(tag = \"object\", rename_all = \"snake_case\")]\n");
        out.push_str("pub enum ");
        out.push_str(&union_name.to_camel_case());
        out.push_str(" {\n");
        for variant_schema in &union_.schema_variants {
            let object_name = meta.spec["components"]["schemas"][&variant_schema]["properties"]
                ["object"]["enum"][0]
                .as_str()
                .unwrap_or_else(|| {
                    meta.spec["components"]["schemas"][&variant_schema]["title"].as_str().unwrap()
                });
            let variant_name = meta.schema_to_rust_type(object_name);
            let type_name = meta.schema_to_rust_type(&variant_schema);
            if variant_name.to_snake_case() != object_name {
                out.push_str("    #[serde(rename = \"");
                out.push_str(object_name);
                out.push_str("\")]\n");
            }
            out.push_str("    ");
            out.push_str(&variant_name);
            out.push('(');
            out.push_str(&type_name);
            out.push_str("),\n");
        }
        out.push_str("}\n");
    }
}

fn gen_variant_name(wire_name: &str, meta: &Metadata) -> String {
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

fn gen_enums(out: &mut String, state: &mut Generated, meta: &Metadata) {
    for (enum_name, enum_) in state.inferred_enums.clone() {
        println!("enum {} {{ ... }}", enum_name);

        out.push('\n');
        out.push_str(&format!(
            "/// An enum representing the possible values of an `{}`'s `{}` field.\n",
            enum_.parent, enum_.field
        ));
        out.push_str("#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]\n");
        out.push_str("#[serde(rename_all = \"snake_case\")]\n");
        out.push_str("pub enum ");
        out.push_str(&enum_name);
        out.push_str(" {\n");
        for wire_name in &enum_.options {
            if wire_name.trim().is_empty() {
                continue;
            }
            let variant_name = gen_variant_name(&wire_name.as_str(), meta);
            if variant_name.trim().is_empty() {
                panic!("unhandled enum variant: {:?}", wire_name)
            }
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
        out.push('\n');
        out.push_str("impl ");
        out.push_str(&enum_name);
        out.push_str(" {\n");
        out.push_str("    pub fn as_str(self) -> &'static str {\n");
        out.push_str("        match self {\n");
        for wire_name in &enum_.options {
            if wire_name.trim().is_empty() {
                continue;
            }
            let variant_name = gen_variant_name(&wire_name.as_str(), meta);
            out.push_str("            ");
            out.push_str(&enum_name);
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
        out.push_str("impl AsRef<str> for ");
        out.push_str(&enum_name);
        out.push_str(" {\n");
        out.push_str("    fn as_ref(&self) -> &str {\n");
        out.push_str("        self.as_str()\n");
        out.push_str("    }\n");
        out.push_str("}\n");
        out.push('\n');
        out.push_str("impl std::fmt::Display for ");
        out.push_str(&enum_name);
        out.push_str(" {\n");
        out.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\n");
        out.push_str("        self.as_str().fmt(f)\n");
        out.push_str("    }\n");
        out.push_str("}\n");
    }
}

fn gen_member_variable_string(schema: &Json) -> Option<String> {
    if let Some(type_) = schema["type"].as_str() {
        let member_type: Option<String>;
        match type_ {
            "integer" => member_type = Some("i32".into()),
            "string" => member_type = Some("String".into()),
            "boolean" => member_type = Some("bool".into()),
            "array" => {
                member_type =
                    Some(format!("Vec<{}>", gen_member_variable_string(&schema["items"]).unwrap()))
            }
            "object" => member_type = None,
            _ => {
                assert!(false, "Do not handle type: {}", type_);
                member_type = None
            }
        }
        member_type
    } else {
        None
    }
}

fn print_doc_from_schema(out: &mut String, schema: &Json, print_level: u8) {
    if let Some(description) = schema["description"].as_str() {
        print_doc_comment(out, description, print_level);
    }
}

fn gen_objects(out: &mut String, state: &mut Generated) {
    let mut generated_objects = state.generated_objects.clone();
    while generated_objects.len() != 0 {
        let key_str: String;
        let value_obj: InferredObject;
        {
            let (key, value) = generated_objects.iter().next().unwrap();
            println!("object: {} -- {:#?}", key, value);
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
        out.push_str("\n");
        print_doc_from_schema(out, &schema, 0);

        if let Some(type_) = schema["type"].as_str() {
            match type_ {
                "object" => {
                    out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
                    out.push_str(&format!("pub struct {} {{\n", key_str));
                    if let Some(prop_map) = schema["properties"].as_object() {
                        let empty_vec = vec![];
                        let required = schema["required"].as_array().unwrap_or(&empty_vec);
                        if !required.iter().all(|val| val.is_string()) {
                            assert!(
                                false,
                                "Required vector is not a vector of strings: {:#?}",
                                required
                            );
                        }
                        for (member_name, member_schema) in prop_map.iter() {
                            let mut is_required = false;
                            if required.iter().any(|val| {
                                let req_str = val.as_str().unwrap();
                                req_str == *member_name
                            }) {
                                is_required = true;
                            }
                            print_doc_from_schema(out, member_schema, 1);
                            if let Some(normal_var) = gen_member_variable_string(member_schema) {
                                write_out_field(out, member_name, &normal_var, is_required);
                            } else {
                                let rust_type = member_name.to_camel_case();
                                write_out_field(out, member_name, &rust_type, is_required);
                                let new_params = InferredObject {
                                    rust_type: rust_type.clone(),
                                    schema: member_schema.clone(),
                                };
                                generated_objects.insert(rust_type, new_params);
                            }
                        }

                        out.push_str("}\n");
                    } else {
                        assert!(false, "Object has no properties: {:#?}", schema);
                    }
                }
                other => assert!(false, "Expected an object here got: {}", other),
            }
        } else if let Some(array) = schema["anyOf"].as_array() {
            out.push_str("#[derive(Clone, Debug, Deserialize, Serialize)]\n");
            out.push_str("#[serde(rename_all = \"snake_case\")]\n");
            out.push_str(&format!("pub enum {} {{\n", key_str));

            let mut index = 0;
            for value in array {
                if let Some(title) = value["title"].as_str() {
                    out.push_str(&format!("    pub {}(", title.to_camel_case()));
                } else {
                    out.push_str(&format!("    pub Alternate{}(", index));
                }
                index += 1;

                if let Some(obj_str) = gen_member_variable_string(&value) {
                    out.push_str(&format!("{}),\n", obj_str));
                } else {
                    let object_name = value["title"].as_str().unwrap();
                    let rust_obj_name = object_name.to_camel_case();

                    out.push_str(&format!("{}),\n", rust_obj_name));
                    let obj_desc =
                        InferredObject { rust_type: rust_obj_name.clone(), schema: value.clone() };
                    generated_objects.insert(object_name.to_camel_case(), obj_desc);
                }
                println!("value: {:#?}", value);
            }
            out.push_str("}\n");
        } else {
            assert!(false, "Schema does not have a type or is not anyOf");
        }
        generated_objects.remove(&key_str);
    }
}

fn gen_extra_object(
    meta: &Metadata,
    object: &str,
    url_finder: &UrlFinder,
    shared_objects: &mut BTreeSet<String>,
) -> String {
    let mut out = String::new();
    let mut state = Generated::default();

    gen_struct(&mut out, &mut state, meta, object, shared_objects, url_finder);

    gen_generated_schemas(&mut out, &mut state, meta, shared_objects);

    gen_inferred_params(&mut out, &mut state, meta, object, shared_objects);

    gen_emitted_structs(&mut out, &mut state, meta, shared_objects);

    gen_unions(&mut out, &mut state, meta);

    gen_enums(&mut out, &mut state, meta);

    gen_objects(&mut out, &mut state);

    let prelude = gen_prelude(&state, meta, object);

    // Done
    prelude + &out
}

fn gen_impl_object(
    meta: &Metadata,
    object: &str,
    url_finder: &UrlFinder,
    shared_objects: &mut BTreeSet<String>,
) -> String {
    let mut out = String::new();
    let mut state = Generated::default();
    let id_type = meta.schema_to_id_type(object);
    let struct_name = meta.schema_to_rust_type(object);
    let schema = &meta.spec["components"]["schemas"][object];
    let fields = match schema["properties"].as_object() {
        Some(some) => some,
        None => return String::new(),
    };

    let object_literal = match fields["object"]["enum"][0].as_str() {
        Some(some) => some,
        None => return String::new(),
    };

    gen_struct(&mut out, &mut state, meta, object, shared_objects, url_finder);

    // Generate request methods
    if let Some(impls) =
        gen_impl_requests(&mut state, meta, object, id_type.as_ref().map(|(x, _)| x.as_str()))
    {
        out.push('\n');
        out.push_str(&impls);
    }

    // Generate an `impl Object` block
    state.use_params.insert("Object");
    out.push('\n');
    out.push_str("impl Object for ");
    out.push_str(&struct_name);
    out.push_str(" {\n");
    out.push_str("    type Id = ");
    if let Some((id_type, _)) = &id_type {
        out.push_str(&id_type);
        out.push_str(";\n");
        out.push_str("    fn id(&self) -> Self::Id {\n        self.id.clone()\n    }\n");
    } else {
        out.push_str("();\n");
        out.push_str("    fn id(&self) -> Self::Id {}\n");
    }
    out.push_str("    fn object(&self) -> &'static str {\n        \"");
    out.push_str(object_literal);
    out.push_str("\"\n    }\n");
    out.push_str("}\n");

    gen_generated_schemas(&mut out, &mut state, meta, shared_objects);

    gen_inferred_params(&mut out, &mut state, meta, object, shared_objects);

    gen_emitted_structs(&mut out, &mut state, meta, shared_objects);

    gen_unions(&mut out, &mut state, meta);

    gen_enums(&mut out, &mut state, meta);

    gen_objects(&mut out, &mut state);

    let prelude = gen_prelude(&state, meta, object);

    // Done
    prelude + &out
}

fn gen_field(
    state: &mut Generated,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &Json,
    required: bool,
    default: bool,
    shared_objects: &mut BTreeSet<String>,
) -> String {
    let mut out = String::new();
    if let Some(doc) = field["description"].as_str() {
        print_doc_comment(&mut out, doc, 1);
    }
    let mut field_rename = field_name.to_snake_case();
    if field_rename == "type" {
        field_rename = "type_".into();
    }
    if field_rename != field_name {
        out.push_str("    #[serde(rename = \"");
        out.push_str(field_name);
        out.push_str("\")]\n");
    }
    let rust_type = gen_field_rust_type(
        state,
        meta,
        object,
        &field_name,
        &field,
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

fn gen_field_rust_type(
    state: &mut Generated,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &Json,
    required: bool,
    default: bool,
    shared_objects: &mut BTreeSet<String>,
) -> String {
    if let Some((use_path, rust_type)) = meta.field_to_rust_type(object, field_name) {
        match use_path {
            "" | "String" => (),
            "Metadata" => {
                state.use_params.insert("Metadata");
            }
            _ => {
                state.use_resources.insert(use_path.into());
            }
        }
        return rust_type.into();
    }
    if field_name == "metadata" {
        state.use_params.insert("Metadata");
        return "Metadata".into();
    } else if (field_name == "currency" || field_name.ends_with("_currency"))
        && field["type"].as_str() == Some("string")
    {
        state.use_resources.insert("Currency".into());
        if !required || field["nullable"].as_bool() == Some(true) {
            return "Option<Currency>".into();
        } else {
            return "Currency".into();
        }
    } else if field_name == "created" {
        state.use_params.insert("Timestamp");
        if !required || field["nullable"].as_bool() == Some(true) {
            return "Option<Timestamp>".into();
        } else {
            return "Timestamp".into();
        }
    }

    let ty = match field["type"].as_str() {
        Some("boolean") => {
            if default {
                // N.B. return immediately; if we want to use `Default` for bool rather than `Option`
                return "bool".into();
            } else {
                "bool".into()
            }
        }
        Some("number") => "f64".into(),
        Some("integer") => infer_integer_type(state, field_name, field["format"].as_str()),
        Some("string") => {
            if let Some(variants) = field["enum"].as_array() {
                let enum_schema = meta.schema_field(object, field_name);
                let enum_name = meta.schema_to_rust_type(&enum_schema);
                let parent_type = meta.schema_to_rust_type(object);
                let enum_ = InferredEnum {
                    parent: parent_type,
                    field: field_name.into(),
                    options: variants.iter().map(|x| x.as_str().unwrap().into()).collect(),
                };
                state.insert_enum(enum_name.clone(), enum_);
                enum_name
            } else {
                "String".into()
            }
        }
        Some("array") => {
            let element = &field["items"];
            let element_type = gen_field_rust_type(
                state,
                meta,
                object,
                field_name,
                element,
                true,
                false,
                shared_objects,
            );
            format!("Vec<{}>", element_type)
        }
        Some("object") => {
            if field["properties"]["object"]["enum"][0].as_str() == Some("list") {
                state.use_params.insert("List");
                let element = &field["properties"]["data"]["items"];
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
            } else {
                let struct_schema = meta.schema_field(object, field_name);
                let struct_name = meta.schema_to_rust_type(&struct_schema);
                let struct_ = InferredStruct { field: field_name.into(), schema: field.clone() };
                state.insert_struct(struct_name.clone(), struct_);
                struct_name
            }
        }
        _ => {
            if let Some(path) = field["$ref"].as_str() {
                let schema_name = path.trim_start_matches("#/components/schemas/");
                let type_name = meta.schema_to_rust_type(schema_name);
                if schema_name != object {
                    if meta.objects.contains(schema_name) {
                        state.use_resources.insert(type_name.clone());
                    } else if meta.dependents.get(schema_name).map(|x| x.len()).unwrap_or(0) > 1 {
                        state.use_resources.insert(type_name.clone());
                        shared_objects.insert(schema_name.to_string());
                    } else if !state.generated_schemas.contains_key(schema_name) {
                        state.generated_schemas.insert(schema_name.into(), false);
                    }
                }
                type_name
            } else if let Some(any_of) =
                field["anyOf"].as_array().or_else(|| field["oneOf"].as_array())
            {
                if any_of.len() == 1
                    || (any_of.len() == 2 && any_of[1]["enum"][0].as_str() == Some(""))
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
                } else if field["x-expansionResources"].is_object() {
                    let ty_ = gen_field_rust_type(
                        state,
                        meta,
                        object,
                        field_name,
                        &json!({
                            "oneOf": Json::Array(field["x-expansionResources"]["oneOf"].as_array().unwrap()
                            .iter()
                            .filter(|v| !v["$ref"].as_str().unwrap().starts_with("#/components/schemas/deleted_"))
                            .cloned()
                            .collect())
                        }),
                        true,
                        false,
                        shared_objects,
                    );
                    state.use_params.insert("Expandable");
                    format!("Expandable<{}>", ty_)
                } else if any_of[0]["title"].as_str() == Some("range_query_specs") {
                    state.use_params.insert("RangeQuery");
                    state.use_params.insert("Timestamp");
                    "RangeQuery<Timestamp>".into()
                } else {
                    //println!("object: {}, field_name: {}", object, field_name);
                    let mut union_addition = field_name.to_owned();
                    union_addition.push_str("_union");
                    let union_schema = meta.schema_field(object, &union_addition);
                    let union_name = meta.schema_to_rust_type(&union_schema);
                    //println!("union_schema: {}, union_name: {}", union_schema, union_name);
                    let union_ = InferredUnion {
                        field: field_name.into(),
                        schema_variants: any_of
                            .iter()
                            .map(|x| {
                                let schema_name = x["$ref"]
                                    .as_str()
                                    .expect(&format!(
                                        "invalid union for `{}.{}`:  {:#?}",
                                        object, field_name, field
                                    ))
                                    .trim_start_matches("#/components/schemas/");
                                let type_name = meta.schema_to_rust_type(schema_name);
                                if meta.objects.contains(schema_name)
                                    || meta
                                        .dependents
                                        .get(schema_name)
                                        .map(|x| x.len())
                                        .unwrap_or(0)
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
            } else {
                panic!("unhandled field type for `{}.{}`: {}\n", object, field_name, field)
            }
        }
    };
    if !required || field["nullable"].as_bool() == Some(true) {
        format!("Option<Box<{}>>", ty)
    } else {
        ty
    }
}

#[derive(Eq, PartialEq, Hash, PartialOrd, Ord)]
enum MethodTypes {
    List,
    Create,
    Retrieve,
    Update,
    Delete,
}

fn gen_impl_requests(
    state: &mut Generated,
    meta: &Metadata,
    object: &str,
    object_id: Option<&str>,
) -> Option<String> {
    let requests = meta.requests.get(object)?;
    let rust_struct = meta.schema_to_rust_type(object);
    println!("impl {} {{ ... }}", rust_struct);

    let mut methods = BTreeMap::new();

    for path in requests {
        let request = &meta.spec["paths"][path];
        let segments = path.trim_start_matches("/v1/").split('/').collect::<Vec<_>>();
        //println!("segments: {:#?}", segments);
        //println!("request: {:#?}", request);

        if let Some(get_request) = &request["get"].as_object() {
            let ok_schema =
                &get_request["responses"]["200"]["content"]["application/json"]["schema"];
            let err_schema =
                &get_request["responses"]["default"]["content"]["application/json"]["schema"];
            if ok_schema.is_null()
                || err_schema["$ref"].as_str() != Some("#/components/schemas/error")
            {
                continue; // skip generating this unusual request (for now...)
            }
            let doc_comment = get_request["description"].as_str().unwrap_or_default();
            if ok_schema["properties"]["object"]["enum"][0].as_str() == Some("list")
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
                    parameters: get_request["parameters"].clone(),
                };
                state.inferred_parameters.insert(params_name.to_snake_case(), params);
                state.use_params.insert("List");

                let mut out = String::new();
                out.push('\n');
                print_doc_comment(&mut out, doc_comment, 1);
                out.push_str("    pub fn list(client: &Client, params: ");
                out.push_str(&params_name);
                out.push_str("<'_>) -> Response<List<");
                out.push_str(&rust_struct);
                out.push_str(">> {\n");
                out.push_str("        client.get_query(\"/");
                out.push_str(&segments.join("/"));
                out.push_str("\", &params)\n");
                out.push_str("    }");
                methods.insert(MethodTypes::List, out);
            } else if segments.len() == 2 && !methods.contains_key(&MethodTypes::Retrieve) {
                let id_param = get_request["parameters"]
                    .as_array()
                    .and_then(|arr| arr.iter().find(|p| p["in"].as_str() == Some("path")));
                let id_param = match id_param {
                    Some(p) => p,
                    None => continue,
                };
                let expand_param = get_request["parameters"]
                    .as_array()
                    .and_then(|arr| arr.iter().find(|p| p["name"].as_str() == Some("expand")));
                if let Some(id_type) = &object_id {
                    assert_eq!(id_param["required"].as_bool(), Some(true));
                    assert_eq!(id_param["style"].as_str(), Some("simple"));

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, doc_comment, 1);
                    out.push_str("    pub fn retrieve(client: &Client, id: &");
                    out.push_str(&id_type);
                    if let Some(param) = expand_param {
                        state.use_params.insert("Expand");
                        assert_eq!(param["in"].as_str(), Some("query"));
                        out.push_str(", expand: &[&str]) -> Response<");
                        out.push_str(&rust_struct);
                        out.push_str("> {\n");
                        out.push_str("        client.get_query(");
                        out.push_str(&format!("&format!(\"/{}/{{}}\", id)", segments[0]));
                        out.push_str(", &Expand { expand })\n");
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

        if let Some(post_request) = &request["post"].as_object() {
            let ok_schema =
                &post_request["responses"]["200"]["content"]["application/json"]["schema"];
            let err_schema =
                &post_request["responses"]["default"]["content"]["application/json"]["schema"];
            if ok_schema.is_null()
                || err_schema["$ref"].as_str() != Some("#/components/schemas/error")
            {
                continue; // skip generating this unusual request (for now...)
            }
            let return_type = if let Some(path) = ok_schema["$ref"].as_str() {
                assert!(ok_schema["nullable"].is_null());
                let schema = path.trim_start_matches("#/components/schemas/");
                meta.schema_to_rust_type(schema)
            } else {
                continue;
            };

            let doc_comment = post_request["description"].as_str().unwrap_or_default();
            let parameter_count = post_request
                .get("parameters")
                .and_then(|p| p.as_array())
                .map(|a| a.len())
                .unwrap_or_default();

            if !methods.contains_key(&MethodTypes::Create)
                && parameter_count == 0
                && (doc_comment.contains("Create")
                    || doc_comment.contains("create")
                    || doc_comment.contains("Adds")
                    || doc_comment.contains("adds"))
            {
                // Just make sure I don't miss anything unexpected
                let query_params: &[_] = post_request
                    .get("parameters")
                    .and_then(|p| p.as_array().map(|x| x.as_ref()))
                    .unwrap_or_default();
                assert!(query_params.is_empty());

                // Construct `parameters` from the request body schema
                let create_schema = &post_request["requestBody"]["content"]
                    ["application/x-www-form-urlencoded"]["schema"];
                let mut create_parameters = Vec::new();
                for (key, value) in create_schema["properties"].as_object().unwrap() {
                    create_parameters.push(json!({
                        "in": "form",
                        "name": key,
                        "description": value["description"],
                        "required": create_schema["required"].as_array().map(|arr| {
                            arr.iter().any(|v| v.as_str() == Some(&key))
                        }),
                        "schema": value,
                        "style": "deepObject"
                    }));
                }
                let params_name = format!("Create{}", rust_struct);
                let params = InferredParams {
                    method: "create".into(),
                    rust_type: params_name.clone(),
                    parameters: Json::Array(create_parameters),
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
                out.push_str("        client.post_form(\"/");
                out.push_str(&segments.join("/"));
                out.push_str("\", &params)\n");
                out.push_str("    }");
                methods.insert(MethodTypes::Create, out);
            } else if !methods.contains_key(&MethodTypes::Update)
                && parameter_count == 1
                && (doc_comment.contains("Update") || doc_comment.contains("update"))
            {
                // Get the id parameter
                let id_param = post_request["parameters"]
                    .as_array()
                    .and_then(|arr| arr.iter().find(|p| p["in"].as_str() == Some("path")));
                let id_param = match id_param {
                    Some(p) => p,
                    None => continue,
                };

                // Construct `parameters` from the request body schema
                let update_schema = &post_request["requestBody"]["content"]
                    ["application/x-www-form-urlencoded"]["schema"];
                let mut update_parameters = Vec::new();
                for (key, value) in update_schema["properties"].as_object().unwrap() {
                    update_parameters.push(json!({
                        "in": "form",
                        "name": key,
                        "description": value["description"],
                        "required": update_schema["required"].as_array().map(|arr| {
                            arr.iter().any(|v| v.as_str() == Some(&key))
                        }),
                        "schema": value,
                        "style": "deepObject"
                    }));
                }
                let params_name = format!("Update{}", rust_struct);
                let params = InferredParams {
                    method: "update".into(),
                    rust_type: params_name.clone(),
                    parameters: Json::Array(update_parameters),
                };
                state.inferred_parameters.insert(params_name.to_snake_case(), params);

                if let Some(id_type) = &object_id {
                    assert_eq!(id_param["required"].as_bool(), Some(true));
                    assert_eq!(id_param["style"].as_str(), Some("simple"));

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, doc_comment, 1);
                    out.push_str("    pub fn update(client: &Client, id: &");
                    out.push_str(&id_type);
                    out.push_str(", params: ");
                    out.push_str(&params_name);
                    out.push_str("<'_>) -> Response<");
                    out.push_str(&return_type);
                    out.push_str("> {\n");
                    out.push_str("        client.post_form(");
                    out.push_str(&format!("&format!(\"/{}/{{}}\", id)", segments[0]));
                    out.push_str(", &params)\n");
                    out.push_str("    }");
                    methods.insert(MethodTypes::Update, out);
                }
            }
        }

        if let Some(delete_request) = &request["delete"].as_object() {
            let ok_schema =
                &delete_request["responses"]["200"]["content"]["application/json"]["schema"];
            let err_schema =
                &delete_request["responses"]["default"]["content"]["application/json"]["schema"];
            if ok_schema.is_null()
                || err_schema["$ref"].as_str() != Some("#/components/schemas/error")
            {
                continue; // skip generating this unusual request (for now...)
            }

            let doc_comment = delete_request["description"].as_str().unwrap_or_default();
            if segments.len() == 2 && !methods.contains_key(&MethodTypes::Delete) {
                let id_param = delete_request["parameters"]
                    .as_array()
                    .and_then(|arr| arr.iter().find(|p| p["in"].as_str() == Some("path")));
                let id_param = match id_param {
                    Some(p) => p,
                    None => continue,
                };
                if let Some(id_type) = &object_id {
                    state.use_params.insert("Deleted");
                    assert_eq!(id_param["required"].as_bool(), Some(true));
                    assert_eq!(id_param["style"].as_str(), Some("simple"));

                    let mut out = String::new();
                    out.push('\n');
                    print_doc_comment(&mut out, doc_comment, 1);
                    out.push_str("    pub fn delete(client: &Client, id: &");
                    out.push_str(&id_type);
                    out.push_str(") -> Response<Deleted<");
                    out.push_str(&id_type);
                    out.push_str(">> {\n");
                    out.push_str("        client.delete(");
                    out.push_str(&format!("&format!(\"/{}/{{}}\", id)", segments[0]));
                    out.push_str(")\n");
                    out.push_str("    }");
                    methods.insert(MethodTypes::Delete, out);
                }
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

fn print_indent(out: &mut String, depth: u8) {
    for _ in 0..depth {
        out.push_str("    ");
    }
}

fn print_doc_comment(out: &mut String, description: &str, depth: u8) {
    if description.trim().is_empty() {
        return;
    }

    let doc = format_doc_comment(description);
    let mut doc_parts = doc.splitn(2, ". ");
    let head = doc_parts.next().unwrap().trim();
    for (i, line) in head.split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
        }
        print_indent(out, depth);
        if !line.is_empty() {
            out.push_str("/// ");
            out.push_str(line);
        } else {
            out.push_str("///");
        }
    }
    if !head.ends_with('.') {
        out.push('.');
    }
    out.push('\n');
    if let Some(tail) = doc_parts.next() {
        print_indent(out, depth);
        out.push_str("///\n");
        for part in tail.split(". ") {
            print_indent(out, depth);
            out.push_str("/// ");
            out.push_str(&part.replace('\n', " ").trim());
            if !part.ends_with('.') {
                out.push('.');
            }
            out.push('\n');
        }
    }
}

fn format_doc_comment(doc: &str) -> String {
    lazy_static! {
        static ref P_TAG: Regex = Regex::new("<p>|</p>").unwrap();
        static ref BR_TAG: Regex = Regex::new("<br ?/?>").unwrap();
        static ref A_DOC_TAG: Regex = Regex::new("<a href=\"/docs/([^\"]+)\">([^<]+)</a>").unwrap();
        static ref A_HASH_TAG: Regex = Regex::new("<a href=\"#([^\"]+)\">([^<]+)</a>").unwrap();
        static ref A_HTTP_TAG: Regex =
            Regex::new("<a href=\"(https?://[^\"]+)\">([^<]+)</a>").unwrap();
        static ref CODE_TAG: Regex = Regex::new("<code>|</code>").unwrap();
        static ref EM_TAG: Regex = Regex::new("<em>|</em>|<i>|</i>").unwrap();
        static ref STRONG_TAG: Regex = Regex::new("<strong>|</strong>|<b>|</b>").unwrap();
        static ref AMOUNT_OPEN_TAG: Regex = Regex::new("<amount>").unwrap();
        static ref AMOUNT_CLOSE_TAG: Regex = Regex::new("</amount>").unwrap();
        static ref CURRENCY_OPEN_TAG: Regex = Regex::new("<currency>").unwrap();
        static ref CURRENCY_CLOSE_TAG: Regex = Regex::new("</currency>").unwrap();
    }
    let doc = P_TAG.replace_all(&doc, "");
    let doc = BR_TAG.replace_all(&doc, "\n");
    let doc = A_DOC_TAG.replace_all(&doc, "[${2}](https://stripe.com/docs/${1})");
    let doc = A_HASH_TAG.replace_all(&doc, "[${2}](https://stripe.com/docs/api#${1})");
    let doc = A_HTTP_TAG.replace_all(&doc, "[$2]($1)");
    let doc = CODE_TAG.replace_all(&doc, "`");
    let doc = EM_TAG.replace_all(&doc, "_");
    let doc = STRONG_TAG.replace_all(&doc, "**");
    let doc = AMOUNT_OPEN_TAG.replace_all(&doc, "");
    let doc = AMOUNT_CLOSE_TAG.replace_all(&doc, "00"); // add cents to get correct "integer" argument
    let doc = CURRENCY_OPEN_TAG.replace_all(&doc, "$"); // add locale formatting (we can only support one easily in our rust docs...)
    let doc = CURRENCY_CLOSE_TAG.replace_all(&doc, "");
    doc.trim().into()
}

fn infer_integer_type(state: &mut Generated, name: &str, format: Option<&str>) -> String {
    let name_snake = name.to_snake_case();
    let name_words = name_snake.split('_').collect::<Vec<_>>();
    if format == Some("unix-time") || name_words.contains(&"date") {
        state.use_params.insert("Timestamp");
        "Timestamp".into()
    } else if name == "monthly_anchor" {
        "u8".into()
    } else if name_words.contains(&"days") {
        "u32".into()
    } else if name_words.contains(&"count")
        || name_words.contains(&"size")
        || name_words.contains(&"quantity")
    {
        "u64".into()
    } else {
        "i64".into()
    }
}

struct UrlFinder {
    html: String,
}

impl UrlFinder {
    fn new() -> Result<Self> {
        let client = reqwest::blocking::Client::builder().user_agent(APP_USER_AGENT).build()?;
        let resp = client.get("https://stripe.com/docs/api").send()?;

        if resp.status().is_success() {
            let text = resp.text()?;
            if text.contains("title: 'Stripe API Reference'") {
                Ok(Self { html: text })
            } else {
                Err(anyhow!("stripe api returned unexpected document"))
            }
        } else {
            eprintln!("{}", resp.text()?);
            Err(anyhow!("request to stripe api returned non-200 status code"))
        }
    }

    fn url_for_object(&self, object: &str) -> Option<String> {
        let object_name = object.replace('.', "_").to_snake_case();
        let doc_url = format!("/{}s/object", object_name);
        if self.html.contains(&doc_url) {
            Some(format!("https://stripe.com/docs/api{}", doc_url))
        } else {
            eprintln!("{} not in html", doc_url);
            None
        }
    }
}
