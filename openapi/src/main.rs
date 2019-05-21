use heck::{CamelCase, SnakeCase};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

fn main() {
    let raw = fs::read_to_string("openapi/spec3.json").unwrap();
    let spec: Value = serde_json::from_str(&raw).unwrap();

    // Compute additional metadata from spec.
    let mut objects = BTreeSet::new();
    let mut dependents: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
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
        let seg = path.trim_start_matches("/v1/").split("/").into_iter().next().unwrap();
        let seg_like = &seg[0..seg.len() - 1];
        if seg.ends_with("s") && objects.contains(seg_like) {
            requests.entry(seg_like).or_default().insert(path.as_str());
        }
    }

    let mut id_renames = BTreeMap::new();
    id_renames.insert("fee_refund", "application_fee_refund");

    let mut schema_renames = BTreeMap::new();
    schema_renames.insert("account_business_profile", "business_profile");
    schema_renames.insert("account_branding_settings", "branding_settings");
    schema_renames.insert("account_card_payments_settings", "card_payments_settings");
    schema_renames.insert("account_dashboard_settings", "dashboard_settings");
    schema_renames.insert("account_decline_charge_on", "decline_charge_on");
    schema_renames.insert("account_external_account", "external_account");
    schema_renames.insert("account_payments_settings", "payments_settings");
    schema_renames.insert("account_payout_settings", "payout_settings");
    schema_renames.insert("account_tos_acceptance", "tos_acceptance");
    schema_renames.insert("fee_refund", "application_fee_refund");
    schema_renames.insert("issuing_authorization_merchant_data", "merchant_data");

    let mut field_types = BTreeMap::new();
    field_types.insert(("account", "business_type"), ("BusinessType", "Option<BusinessType>"));
    field_types.insert(("account", "type"), ("AccountType", "AccountType"));
    field_types.insert(
        ("account_capabilities", "card_payments"),
        ("CapabilityStatus", "CapabilityStatus"),
    );
    field_types.insert(
        ("account_capabilities", "legacy_payments"),
        ("CapabilityStatus", "CapabilityStatus"),
    );
    field_types.insert(
        ("account_capabilities", "platform_payments"),
        ("CapabilityStatus", "CapabilityStatus"),
    );
    field_types.insert(
        ("balance_transaction", "status"),
        ("BalanceTransactionStatus", "BalanceTransactionStatus"),
    );
    field_types.insert(("fee", "type"), ("FeeType", "FeeType"));
    field_types.insert(
        ("bank_account", "account_holder_type"),
        ("AccountHolderType", "Option<AccountHolderType>"),
    );
    field_types
        .insert(("bank_account", "status"), ("BankAccountStatus", "Option<BankAccountStatus>"));
    field_types.insert(
        ("issuing.authorization", "authorization_method"),
        ("IssuingAuthorizationMethod", "IssuingAuthorizationMethod"),
    );
    field_types.insert(
        ("issuing.authorization", "wallet_provider"),
        ("WalletProvider", "Option<WalletProvider>"),
    );
    field_types.insert(
        ("issuing_authorization_request", "reason"),
        ("IssuingAuthorizationReason", "IssuingAuthorizationReason"),
    );
    field_types.insert(
        ("issuing_authorization_verification_data", "address_line1_check"),
        ("IssuingAuthorizationCheck", "IssuingAuthorizationCheck"),
    );
    field_types.insert(
        ("issuing_authorization_verification_data", "address_zip_check"),
        ("IssuingAuthorizationCheck", "IssuingAuthorizationCheck"),
    );
    field_types.insert(
        ("issuing_authorization_verification_data", "cvc_check"),
        ("IssuingAuthorizationCheck", "IssuingAuthorizationCheck"),
    );
    field_types.insert(("issuing.card", "brand"), ("CardBrand", "CardBrand"));
    field_types.insert(("issuing.card", "type"), ("IssuingCardType", "IssuingCardType"));
    field_types.insert(
        ("issuing_card_authorization_controls", "allowed_categories"),
        ("String", "Option<Vec<String>>"),
    );
    field_types.insert(
        ("issuing_card_authorization_controls", "blocked_categories"),
        ("String", "Option<Vec<String>>"),
    );
    field_types.insert(
        ("issuing_card_authorization_controls", "spending_limits"),
        ("SpendingLimit", "Option<Vec<SpendingLimit>>"),
    );
    field_types.insert(
        ("issuing_card_shipping", "status"),
        ("IssuingCardShippingStatus", "Option<IssuingCardShippingStatus>"),
    );
    field_types.insert(
        ("issuing_card_shipping", "type"),
        ("IssuingCardShippingType", "IssuingCardShippingType"),
    );
    field_types
        .insert(("issuing.cardholder", "type"), ("IssuingCardholderType", "IssuingCardholderType"));
    field_types.insert(
        ("issuing_cardholder_authorization_controls", "allowed_categories"),
        ("String", "Option<Vec<String>>"),
    );
    field_types.insert(
        ("issuing_cardholder_authorization_controls", "blocked_categories"),
        ("String", "Option<Vec<String>>"),
    );
    field_types.insert(
        ("issuing_cardholder_authorization_controls", "spending_limits"),
        ("SpendingLimit", "Option<Vec<SpendingLimit>>"),
    );
    field_types
        .insert(("issuing.dispute", "reason"), ("IssuingDisputeReason", "IssuingDisputeReason"));
    field_types
        .insert(("issuing.dispute", "status"), ("IssuingDisputeStatus", "IssuingDisputeStatus"));
    field_types.insert(
        ("issuing.transaction", "type"),
        ("IssuingTransactionType", "IssuingTransactionType"),
    );

    // Generate files
    let meta = Metadata {
        spec: &spec,
        objects,
        dependents,
        requests,
        id_renames,
        schema_renames,
        field_types,
    };
    for object in &meta.objects {
        if object.starts_with("deleted_") {
            continue;
        }

        // Generate the types for the object
        let out = gen_impl_object(&meta, object);
        fs::write(
            &format!("openapi/out/{}.rs", object.replace('.', "_").to_snake_case()),
            out.as_bytes(),
        )
        .unwrap();
    }
}

struct Metadata<'a> {
    spec: &'a Value,
    /// The set of schemas which should implement `Object`.
    /// These have both an `id` property and on `object` property.
    objects: BTreeSet<&'a str>,
    /// A one to many map of schema to depending types.
    dependents: BTreeMap<&'a str, BTreeSet<&'a str>>,
    /// A one to many map of _objects_ to requests which should be
    /// implemented for that object.
    ///
    /// This is typically determined by the first segment in the path.
    requests: BTreeMap<&'a str, BTreeSet<&'a str>>,
    /// How a particular id type shouldd be renamed.
    id_renames: BTreeMap<&'a str, &'a str>,
    /// How a particular schema should be renamed.
    schema_renames: BTreeMap<&'a str, &'a str>,
    /// An override for the rust-type of a particular object/field pair.
    field_types: BTreeMap<(&'a str, &'a str), (&'a str, &'a str)>,
}

impl<'a> Metadata<'a> {
    fn id_to_rust_type(&self, id: &str) -> String {
        if let Some(rename) = self.id_renames.get(&id) {
            rename.to_camel_case() + "Id"
        } else {
            id.replace('.', "_").to_camel_case() + "Id"
        }
    }
    fn schema_to_rust_type(&self, schema: &str) -> String {
        if let Some(rename) = self.schema_renames.get(&schema) {
            rename.to_camel_case()
        } else {
            schema.replace('.', "_").to_camel_case()
        }
    }
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
    let struct_name = meta.schema_to_rust_type(object);
    let schema = &meta.spec["components"]["schemas"][object];
    let schema_title = schema["title"].as_str().unwrap();
    let deleted_schema = &meta.spec["components"]["schemas"][format!("deleted_{}", object)];
    let id_type = if !schema["properties"]["id"].is_null() {
        schema["x-resourceId"].as_str().map(|id| meta.id_to_rust_type(id))
    } else {
        None
    };
    let fields = match schema["properties"].as_object() {
        Some(some) => some,
        None => return String::new(),
    };
    let object_literal = match fields["object"]["enum"][0].as_str() {
        Some(some) => some,
        None => return String::new(),
    };

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
            out.push_str("    deleted: bool,\n");
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
            &mut impl_,
            meta,
            object,
            &key,
            &field,
            required && !force_optional,
        ));
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
        out.push_str("    fn id(&self) -> &Self::Id {\n        &()\n    }\n");
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
            out.push_str(&gen_field(&mut impl_, meta, &schema_name, &key, &field, required));
        }
        out.push_str("}\n");

        // Set the schema to generated
        *impl_.generated_schemas.entry(schema_name).or_default() = true;
    }

    for (struct_name, struct_) in impl_.inferred_structs.clone() {
        let fields = match struct_.schema["properties"].as_object() {
            Some(some) => some,
            None => {
                // TODO: Handle these objects
                // eprintln!("warn: {} has no properties ({})", struct_name, struct_.schema);
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
                &mut impl_,
                meta,
                &struct_name.to_snake_case(),
                &key,
                &field,
                required,
            ));
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
        for variant_schema in &union_.schema_variants {
            let object_name = meta.spec["components"]["schemas"][&variant_schema]["properties"]
                ["object"]["enum"][0]
                .as_str()
                .unwrap();
            let variant_name = meta.schema_to_rust_type(object_name);
            let type_name = meta.schema_to_rust_type(&variant_schema);
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
        out.push_str(&format!(
            "/// An enum representing the possible values of an `{}`'s `{}` field.\n",
            enum_.parent, enum_.field
        ));
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
    required: bool,
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
    let rust_type = gen_field_rust_type(impl_, meta, object, &field_name, &field, required);
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
    impl_: &mut ImplMetadata,
    meta: &Metadata,
    object: &str,
    field_name: &str,
    field: &Value,
    required: bool,
) -> String {
    if let Some(&(use_path, rust_type)) = meta.field_types.get(&(object, field_name)) {
        match use_path {
            "String" => (),
            _ => {
                impl_.use_resources.insert(use_path.into());
            }
        }
        return rust_type.into();
    }
    if field_name == "metadata" {
        impl_.use_params.insert("Metadata");
        return "Metadata".into();
    } else if field_name == "currency" || field_name.ends_with("_currency") {
        impl_.use_resources.insert("Currency".into());
        if !required || field["nullable"].as_bool() == Some(true) {
            return "Option<Currency>".into();
        } else {
            return "Currency".into();
        }
    } else if field_name == "created" {
        impl_.use_params.insert("Timestamp");
        if !required || field["nullable"].as_bool() == Some(true) {
            return "Option<Timestamp>".into();
        } else {
            return "Timestamp".into();
        }
    }

    let ty = match field["type"].as_str() {
        Some("boolean") => {
            // N.B. return immediately; we use `Default` for bool rather than `Option`
            return "bool".into();
        }
        Some("number") => "f64".into(),
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
                let enum_schema = format!("{}_{}", object, field_name).to_snake_case();
                let enum_name = meta.schema_to_rust_type(&enum_schema);
                let parent_type = meta.schema_to_rust_type(object);
                let enum_ = InferredEnum {
                    parent: parent_type,
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
            let element_type = gen_field_rust_type(impl_, meta, object, field_name, element, true);
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
                    gen_field_rust_type(impl_, meta, object, element_field_name, element, true);

                // N.B. return immediately; we use `Default` for list rather than `Option`
                return format!("List<{}>", element_type);
            } else {
                let struct_name =
                    format!("{}_{}", object.replace('.', "_"), field_name).to_camel_case();
                let struct_ = InferredStruct { field: field_name.into(), schema: field.clone() };
                impl_.inferred_structs.insert(struct_name.clone(), struct_);
                struct_name
            }
        }
        _ => {
            if let Some(path) = field["$ref"].as_str() {
                let schema_name = path.trim_start_matches("#/components/schemas/");
                let type_name = meta.schema_to_rust_type(schema_name);
                if schema_name != object {
                    if meta.objects.contains(schema_name)
                        || meta.dependents.get(schema_name).map(|x| x.len()).unwrap_or(0) > 1
                    {
                        impl_.use_resources.insert(type_name.clone());
                    } else if !impl_.generated_schemas.contains_key(schema_name) {
                        impl_.generated_schemas.insert(schema_name.into(), false);
                    }
                }
                type_name
            } else if let Some(any_of) = field["anyOf"].as_array().or(field["oneOf"].as_array()) {
                if any_of.len() == 1 {
                    gen_field_rust_type(impl_, meta, object, field_name, &any_of[0], true)
                } else if field["x-expansionResources"].is_object() {
                    let ty_ = gen_field_rust_type(
                        impl_,
                        meta,
                        object,
                        field_name,
                        &serde_json::json!({
                            "oneOf": Value::Array(field["x-expansionResources"]["oneOf"].as_array().unwrap()
                            .iter()
                            .filter(|v| !v["$ref"].as_str().unwrap().starts_with("#/components/schemas/deleted_"))
                            .cloned()
                            .collect())
                        }),
                        true,
                    );
                    impl_.use_params.insert("Expandable");
                    format!("Expandable<{}>", ty_)
                } else {
                    let union_schema = format!("{}_{}", object, field_name).to_snake_case();
                    let union_name = meta.schema_to_rust_type(&union_schema);
                    let union_ = InferredUnion {
                        field: field_name.into(),
                        schema_variants: any_of
                            .into_iter()
                            .map(|x| {
                                let schema_name = x["$ref"]
                                    .as_str()
                                    .unwrap()
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
    if !required || field["nullable"].as_bool() == Some(true) {
        format!("Option<{}>", ty)
    } else {
        ty
    }
}

fn print_doc_comment(out: &mut String, comment: &str) {
    let mut doc_parts = comment.splitn(2, ". ");
    let head = doc_parts.next().unwrap();
    out.push_str("    /// ");
    out.push_str(&head.replace('\n', "\n    /// "));
    if !head.ends_with(".") {
        out.push('.');
    }
    out.push('\n');
    if let Some(tail) = doc_parts.next() {
        out.push_str("    ///\n");
        for part in tail.split(". ") {
            out.push_str("    /// ");
            out.push_str(&part.replace('\n', "\n    /// "));
            if !part.ends_with(".") {
                out.push('.');
            }
            out.push('\n');
        }
    }
}

fn check_object_doc_url(object: &str) -> Option<String> {
    let doc_url =
        format!("https://stripe.com/docs/api/{}s/object", object.replace('.', "_").to_snake_case());
    let cache_file =
        format!("openapi/cache/{}_object.html", object.replace('.', "_").to_snake_case());
    if let Some(cached) = fs::read_to_string(&cache_file).ok() {
        return if cached.is_empty() { None } else { Some(doc_url) };
    }

    // Make a request to the stripe docs to see if the path exists
    if let Ok(mut resp) = reqwest::get(&doc_url) {
        if resp.status().is_success() {
            let text = resp.text().unwrap();
            if text.contains("<title>Stripe API Reference") && text.contains("object</title>") {
                fs::write(&cache_file, text.as_bytes()).unwrap();
                return Some(doc_url);
            } else {
                panic!("fatal: documentation response didn't match the expected format.");
            }
        }
    }
    eprintln!("warning: could not determine doc_url for object `{}`", object);
    fs::write(&cache_file, "").unwrap();
    None
}
